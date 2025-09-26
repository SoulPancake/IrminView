open Lwt.Syntax
open Cohttp_lwt_unix

(* Default configuration *)
let default_port = 8080
let default_store_path = "/data/irmin_store"

(* CORS headers for browser requests *)
let cors_headers = [
  ("Access-Control-Allow-Origin", "*");
  ("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
  ("Access-Control-Allow-Headers", "Content-Type");
]

(* Helper to create JSON response with CORS headers *)
let json_response ?(status=`OK) json_string =
  let headers = Cohttp.Header.of_list (("Content-Type", "application/json") :: cors_headers) in
  Server.respond_string ~status ~headers ~body:json_string ()

(* Helper to create error response *)
let error_response ?(status=`Internal_server_error) message =
  let error_json = Printf.sprintf {|{"error": "%s"}|} message in
  json_response ~status error_json

(* Route handlers *)
let handle_tree store_path _req =
  let* result = Irmin_bridge.get_tree_json store_path in
  json_response result

let handle_commits store_path _req =
  let* result = Irmin_bridge.get_commits_json store_path in
  json_response result

let handle_branches store_path _req =
  let* result = Irmin_bridge.get_branches_json store_path in
  json_response result

let handle_search store_path req =
  let uri = Cohttp.Request.uri req in
  let query = Uri.get_query_param uri "q" in
  match query with
  | Some q ->
      let* result = Irmin_bridge.search_keys_json store_path q in
      json_response result
  | None ->
      error_response ~status:`Bad_request "Missing query parameter 'q'"

let handle_diff store_path req =
  let uri = Cohttp.Request.uri req in
  let from_commit = Uri.get_query_param uri "from" in
  let to_commit = Uri.get_query_param uri "to" in
  match (from_commit, to_commit) with
  | (Some from_c, Some to_c) ->
      let* result = Irmin_bridge.get_diff_json store_path from_c to_c in
      json_response result
  | _ ->
      error_response ~status:`Bad_request "Missing 'from' or 'to' query parameters"

let handle_options _store_path _req =
  let headers = Cohttp.Header.of_list cors_headers in
  Server.respond_string ~status ~headers ~body:"" ()

let handle_health _store_path _req =
  let health_json = {|{"status":"healthy","service":"irmin-bridge-server"}|} in
  json_response health_json

(* Main request router *)
let callback store_path _conn req _body =
  let uri = Cohttp.Request.uri req in
  let path = Uri.path uri in
  let meth = Cohttp.Request.meth req in
  
  Printf.printf "%s %s\n%!" (Cohttp.Code.string_of_method meth) path;
  
  try
    match (meth, path) with
    | (`OPTIONS, _) -> handle_options store_path req
    | (`GET, "/health") -> handle_health store_path req
    | (`GET, "/api/tree") -> handle_tree store_path req
    | (`GET, "/api/commits") -> handle_commits store_path req
    | (`GET, "/api/branches") -> handle_branches store_path req
    | (`GET, "/api/search") -> handle_search store_path req
    | (`GET, "/api/diff") -> handle_diff store_path req
    | _ -> 
        error_response ~status:`Not_found "Endpoint not found"
  with
  | exn ->
      let error_msg = Printexc.to_string exn in
      Printf.eprintf "Error handling request: %s\n%!" error_msg;
      error_response ~status:`Internal_server_error error_msg

(* Command line argument parsing *)
let port_arg =
  let doc = "Port to listen on" in
  Cmdliner.Arg.(value & opt int default_port & info ["p"; "port"] ~docv:"PORT" ~doc)

let store_path_arg =
  let doc = "Path to the Irmin store" in
  Cmdliner.Arg.(value & opt string default_store_path & info ["s"; "store"] ~docv:"PATH" ~doc)

let server_cmd =
  let doc = "Start the Irmin bridge HTTP server" in
  let info = Cmdliner.Cmd.info "irmin-bridge-server" ~doc in
  let term = 
    Cmdliner.Term.(const (fun port store_path ->
      Printf.printf "Starting Irmin bridge server on port %d\n" port;
      Printf.printf "Using Irmin store at: %s\n" store_path;
      Printf.printf "Available endpoints:\n";
      Printf.printf "  GET /health - Health check\n";
      Printf.printf "  GET /api/tree - Get tree structure\n";
      Printf.printf "  GET /api/commits - Get commit history\n";
      Printf.printf "  GET /api/branches - Get branches\n";
      Printf.printf "  GET /api/search?q=<query> - Search keys\n";
      Printf.printf "  GET /api/diff?from=<hash>&to=<hash> - Get diff\n";
      Printf.printf "\n";
      
      let callback = callback store_path in
      let server = Server.create ~mode:(`TCP (`Port port)) (Server.make ~callback ()) in
      Lwt_main.run server
    ) $ port_arg $ store_path_arg) in
  Cmdliner.Cmd.v info term

let () = exit (Cmdliner.Cmd.eval server_cmd)