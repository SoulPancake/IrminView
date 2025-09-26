(lang dune 3.0)

(name irmin-bridge)

(package
 (name irmin-bridge)
 (depends ocaml dune irmin irmin-fs irmin-git yojson lwt cmdliner))