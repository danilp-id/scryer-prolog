% for debugging
% consult(http), run.

:- use_module(library(http/http_server)).
:- use_module(library(pio)).
:- use_module(library(dcgs)).

text_handler(Request, Response) :-
  http_status_code(Response, 200),
  http_body(Response, text("Welcome to Scryer Prolog!")).

parameter_handler(User, Request, Response) :-
  http_body(Response, text(User)).

run:-
  %phrase_from_file(seq(TlsKey), "cert/private.key"),
  %phrase_from_file(seq(TlsCert), "cert/certificate.crt"),

  http_listen(7890, [
    get(echo, text_handler),                 % GET /echo
    post(user/User, parameter_handler(User)) % POST /user/<User>
  ], [tls_key("cert/private.key"), tls_cert("cert/certificate.crt")]).