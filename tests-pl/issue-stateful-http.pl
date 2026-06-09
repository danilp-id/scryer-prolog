%:- use_module(library(http/http_server)).
:- use_module(library(clpz)).

run :- http_listen(7890, [
  get(/, val),
  get(inc, inc),
  get(dec, dec),
  get(echo/Echo, echo(Echo))
], [initial_state(0)]).

foo(X, X, Y, Y).

val_resp(Response, S) :-
    nl, write(S), nl,
    number_chars(S, SC),
    http_body(Response, text(SC)).

val(_, Response, S) :- val_resp(Response, S).

inc(_, Response, S0, S) :-
    S #= S0 + 1,
    val_resp(Response, S).

dec(_, Response, S0, S) :-
    S #= S0 - 1,
    val_resp(Response, S).

echo(Echo, _, Response) :-
    http_body(Response, text(Echo)).