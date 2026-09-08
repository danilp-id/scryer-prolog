% for debugging
% consult(http), run.

:- use_module(library(http/http_server)).

text_handler(Request, Response) :-
  http_status_code(Response, 200),
  http_body(Response, text("Welcome to Scryer Prolog!")).

parameter_handler(User, Request, Response) :-
  http_body(Response, text(User)).

run:-
  http_listen(7890, [
    get(echo, text_handler),                 % GET /echo
    post(user/User, parameter_handler(User)) % POST /user/<User>
  ]).