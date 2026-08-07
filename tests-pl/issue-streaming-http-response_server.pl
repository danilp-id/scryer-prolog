:- use_module(library(http/http_server)).

% debug
:- use_module(library(format)).

run :- http_listen(8472, [
  get(/, stream),
  get(text, text),
  get(bytes, bytes),
  get(file, file)
]).

stream(_, Response) :-
  write("1 "), portray_clause(Response),
  http_body(Response, text("hello")),
  write("2 "), portray_clause(Response),
  http_body(Response, text("world")),
  write("3 "), portray_clause(Response).

text(_, Response) :-
  http_body(Response, text("some text")).

bytes(_, Response) :-
  http_body(Response, bytes("some bytes")).

file(_, Response) :-
  http_body(Response, file("issue-streaming-http-response_server.pl")).