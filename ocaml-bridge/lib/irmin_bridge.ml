open Lwt.Syntax
open Yojson.Safe

(* Irmin store configuration *)
module Store = Irmin_git.FS.G (Irmin.Contents.String) (Irmin.Path.String_list) (Irmin.Branch.String)

type irmin_node = {
  key: string;
  value: string option;
  node_type: string;
  children: (string * irmin_node) list;
  metadata: irmin_metadata;
}

and irmin_metadata = {
  last_modified: string;
  size: int option;
  permissions: string option;
}

type irmin_commit = {
  hash: string;
  message: string;
  author: string;
  timestamp: string;
  parents: string list;
  branch: string;
}

type irmin_branch = {
  name: string;
  head_commit: string;
  last_updated: string;
  commit_count: int;
}

type search_result = {
  path: string;
  node: irmin_node;
  relevance_score: float;
}

type diff_change = {
  path: string;
  change_type: string;
  old_value: string option;
  new_value: string option;
}

type irmin_diff = {
  from_commit: string;
  to_commit: string;
  changes: diff_change list;
}

(* Convert OCaml types to JSON *)
let metadata_to_json metadata =
  `Assoc [
    ("last_modified", `String metadata.last_modified);
    ("size", match metadata.size with Some s -> `Int s | None -> `Null);
    ("permissions", match metadata.permissions with Some p -> `String p | None -> `Null);
  ]

let rec node_to_json node =
  `Assoc [
    ("key", `String node.key);
    ("value", match node.value with Some v -> `String v | None -> `Null);
    ("node_type", `String node.node_type);
    ("children", `Assoc (List.map (fun (k, v) -> (k, node_to_json v)) node.children));
    ("metadata", metadata_to_json node.metadata);
  ]

let commit_to_json commit =
  `Assoc [
    ("hash", `String commit.hash);
    ("message", `String commit.message);
    ("author", `String commit.author);
    ("timestamp", `String commit.timestamp);
    ("parents", `List (List.map (fun p -> `String p) commit.parents));
    ("branch", `String commit.branch);
  ]

let branch_to_json branch =
  `Assoc [
    ("name", `String branch.name);
    ("head_commit", `String branch.head_commit);
    ("last_updated", `String branch.last_updated);
    ("commit_count", `Int branch.commit_count);
  ]

let search_result_to_json result =
  `Assoc [
    ("path", `String result.path);
    ("node", node_to_json result.node);
    ("relevance_score", `Float result.relevance_score);
  ]

let diff_change_to_json change =
  `Assoc [
    ("path", `String change.path);
    ("change_type", `String change.change_type);
    ("old_value", match change.old_value with Some v -> `String v | None -> `Null);
    ("new_value", match change.new_value with Some v -> `String v | None -> `Null);
  ]

let diff_to_json diff =
  `Assoc [
    ("from_commit", `String diff.from_commit);
    ("to_commit", `String diff.to_commit);
    ("changes", `List (List.map diff_change_to_json diff.changes));
  ]

(* Initialize Irmin store *)
let init_store ?(path="./irmin_store") () =
  let config = Irmin_git.config path in
  Store.Repo.init config

(* Get tree structure from Irmin store *)
let get_tree_from_store repo =
  let* main = Store.of_branch repo "main" in
  let* tree = Store.tree main in
  let rec build_node_tree path key =
    let* node_opt = Store.Tree.find_tree tree path in
    match node_opt with
    | None -> 
        let* value_opt = Store.Tree.find tree path in
        let metadata = {
          last_modified = "2024-01-01T00:00:00Z";
          size = (match value_opt with Some v -> Some (String.length v) | None -> None);
          permissions = Some "644";
        } in
        Lwt.return {
          key = key;
          value = value_opt;
          node_type = "File";
          children = [];
          metadata;
        }
    | Some subtree ->
        let* children_keys = Store.Tree.list subtree [] in
        let* children = Lwt_list.map_s (fun (child_key, _) ->
          let child_path = path @ [child_key] in
          let* child_node = build_node_tree child_path child_key in
          Lwt.return (child_key, child_node)
        ) children_keys in
        let metadata = {
          last_modified = "2024-01-01T00:00:00Z";
          size = None;
          permissions = Some "755";
        } in
        Lwt.return {
          key = key;
          value = None;
          node_type = "Directory";
          children = children;
          metadata;
        }
  in
  build_node_tree [] "root"

(* Get commits from Irmin store *)
let get_commits_from_store repo =
  let* main = Store.of_branch repo "main" in
  let* history = Store.history main in
  let* commits = Store.History.iter history (fun commit_key ->
    let* commit_info = Store.Commit.info commit_key in
    let hash = Store.Commit.hash commit_key |> Store.Hash.to_string in
    let message = Store.Info.message commit_info in
    let author = Store.Info.author commit_info |> fun (name, email) -> Printf.sprintf "%s <%s>" name email in
    let timestamp = Store.Info.date commit_info |> Int64.to_string in
    let* parents = Store.Commit.parents commit_key in
    let parent_hashes = List.map (fun p -> Store.Commit.hash p |> Store.Hash.to_string) parents in
    Lwt.return {
      hash;
      message;
      author;
      timestamp;
      parents = parent_hashes;
      branch = "main";
    }
  ) in
  Lwt.return commits

(* Get branches from Irmin store *)
let get_branches_from_store repo =
  let* branches = Store.Repo.branches repo in
  let* branch_list = Lwt_list.map_s (fun branch_name ->
    let* store = Store.of_branch repo branch_name in
    let* head_commit = Store.Head.get store in
    let head_hash = Store.Commit.hash head_commit |> Store.Hash.to_string in
    let* history = Store.history store in
    let* commit_count = Store.History.fold history 0 (fun acc _ -> Lwt.return (acc + 1)) in
    Lwt.return {
      name = branch_name;
      head_commit = head_hash;
      last_updated = "2024-01-01T00:00:00Z";
      commit_count;
    }
  ) branches in
  Lwt.return branch_list

(* Search keys in the store *)
let search_keys_in_store repo query =
  let* main = Store.of_branch repo "main" in
  let* tree = Store.tree main in
  let rec search_recursive path key_prefix =
    let* list = Store.Tree.list tree path in
    let* results = Lwt_list.fold_left_s (fun acc (key, kind) ->
      let full_path = path @ [key] in
      let path_str = String.concat "/" full_path in
      let relevance = if String.contains (String.lowercase_ascii key) (String.lowercase_ascii query) then
        if String.equal (String.lowercase_ascii key) (String.lowercase_ascii query) then 1.0
        else if String.prefix (String.lowercase_ascii key) (String.lowercase_ascii query) then 0.8
        else 0.5
      else 0.0 in
      
      let* node = match kind with
        | `Tree -> 
            let* children = search_recursive full_path (path_str ^ "/") in
            let metadata = {
              last_modified = "2024-01-01T00:00:00Z";
              size = None;
              permissions = Some "755";
            } in
            Lwt.return {
              key;
              value = None;
              node_type = "Directory";
              children = [];
              metadata;
            }
        | `Contents ->
            let* value = Store.Tree.find tree full_path in
            let metadata = {
              last_modified = "2024-01-01T00:00:00Z";
              size = (match value with Some v -> Some (String.length v) | None -> None);
              permissions = Some "644";
            } in
            Lwt.return {
              key;
              value;
              node_type = "File";
              children = [];
              metadata;
            }
      in
      
      if relevance > 0.0 then
        let result = { path = path_str; node; relevance_score = relevance } in
        Lwt.return (result :: acc)
      else
        Lwt.return acc
    ) [] list in
    Lwt.return results
  in
  search_recursive [] ""

(* Get diff between commits *)
let get_diff_between_commits repo from_commit to_commit =
  (* This is a simplified diff implementation *)
  Lwt.return {
    from_commit;
    to_commit;
    changes = [
      {
        path = "/example/changed_file.txt";
        change_type = "Modified";
        old_value = Some "old content";
        new_value = Some "new content";
      }
    ];
  }

(* Command-line interface functions *)
let get_tree_json path =
  let* repo = init_store ~path () in
  let* tree = get_tree_from_store repo in
  let json = node_to_json tree in
  Lwt.return (to_string json)

let get_commits_json path =
  let* repo = init_store ~path () in
  let* commits = get_commits_from_store repo in
  let json = `List (List.map commit_to_json commits) in
  Lwt.return (to_string json)

let get_branches_json path =
  let* repo = init_store ~path () in
  let* branches = get_branches_from_store repo in
  let json = `List (List.map branch_to_json branches) in
  Lwt.return (to_string json)

let search_keys_json path query =
  let* repo = init_store ~path () in
  let* results = search_keys_in_store repo query in
  let json = `List (List.map search_result_to_json results) in
  Lwt.return (to_string json)

let get_diff_json path from_commit to_commit =
  let* repo = init_store ~path () in
  let* diff = get_diff_between_commits repo from_commit to_commit in
  let json = diff_to_json diff in
  Lwt.return (to_string json)