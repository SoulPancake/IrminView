open Cmdliner
open Lwt.Syntax

let default_path = "./irmin_store"

let path_arg =
  let doc = "Path to the Irmin store" in
  Arg.(value & opt string default_path & info ["p"; "path"] ~docv:"PATH" ~doc)

let get_tree_cmd =
  let doc = "Get the tree structure from the Irmin store" in
  let info = Cmd.info "tree" ~doc in
  let term = 
    Term.(const (fun path ->
      Lwt_main.run (
        let* json = Irmin_bridge.get_tree_json path in
        Lwt_io.printf "%s\n" json
      )
    ) $ path_arg) in
  Cmd.v info term

let get_commits_cmd =
  let doc = "Get commits from the Irmin store" in
  let info = Cmd.info "commits" ~doc in
  let term =
    Term.(const (fun path ->
      Lwt_main.run (
        let* json = Irmin_bridge.get_commits_json path in
        Lwt_io.printf "%s\n" json
      )
    ) $ path_arg) in
  Cmd.v info term

let get_branches_cmd =
  let doc = "Get branches from the Irmin store" in
  let info = Cmd.info "branches" ~doc in
  let term =
    Term.(const (fun path ->
      Lwt_main.run (
        let* json = Irmin_bridge.get_branches_json path in
        Lwt_io.printf "%s\n" json
      )
    ) $ path_arg) in
  Cmd.v info term

let search_keys_cmd =
  let query_arg =
    let doc = "Search query" in
    Arg.(required & pos 0 (some string) None & info [] ~docv:"QUERY" ~doc) in
  let doc = "Search for keys in the Irmin store" in
  let info = Cmd.info "search" ~doc in
  let term =
    Term.(const (fun path query ->
      Lwt_main.run (
        let* json = Irmin_bridge.search_keys_json path query in
        Lwt_io.printf "%s\n" json
      )
    ) $ path_arg $ query_arg) in
  Cmd.v info term

let get_diff_cmd =
  let from_arg =
    let doc = "From commit hash" in
    Arg.(required & pos 0 (some string) None & info [] ~docv:"FROM" ~doc) in
  let to_arg =
    let doc = "To commit hash" in
    Arg.(required & pos 1 (some string) None & info [] ~docv:"TO" ~doc) in
  let doc = "Get diff between two commits" in
  let info = Cmd.info "diff" ~doc in
  let term =
    Term.(const (fun path from_commit to_commit ->
      Lwt_main.run (
        let* json = Irmin_bridge.get_diff_json path from_commit to_commit in
        Lwt_io.printf "%s\n" json
      )
    ) $ path_arg $ from_arg $ to_arg) in
  Cmd.v info term

let default_cmd =
  let doc = "Irmin bridge CLI for IrminView" in
  let info = Cmd.info "irmin-bridge-cli" ~doc in
  let term = Term.(ret (const (`Help (`Pager, None)))) in
  Cmd.v info term

let cmds = [get_tree_cmd; get_commits_cmd; get_branches_cmd; search_keys_cmd; get_diff_cmd]

let () =
  let cmd = Cmd.group default_cmd cmds in
  exit (Cmd.eval cmd)