:- use_module(library(http/http_server)).
:- use_module(library(clpz)).
:- use_module(library(charsio)).
:- use_module(library(lists)).

run :- run(false, 0).

run(CatchErrors, State) :- http_listen(7890, [
  get(/, val),
  get(inc, inc),
  get(dec, dec),
  get(times2, times2),
  get(error, my_error),
  get(missing, missing),
  get(echo/Echo, echo(Echo))
], [initial_state(State), catch_errors(CatchErrors)]).

run_uninterrupted :- run(true, 0).

% NOTE: do not use raw throw/1 if you wish the state to be persisted on errors, see library(error).
my_error(_, _) :- resource_error("all cookies expired").

% does not use state
echo(Echo, _, Response) :-
    http_body(Response, text(Echo)).

val_resp(Response, S) :-
    nl, write(S), nl,
    number_chars(S, SC),
    http_body(Response, text(SC)).

% can only read state
val(_, Response, S) :- val_resp(Response, S).

% can read and modify state
inc(_, Response, S0, S) :-
    S #= S0 + 1,
    val_resp(Response, S).

times2(_, Response, S0, S) :-
    S #= S0*2,
    val_resp(Response, S).

dec(_, Response, S0, S) :-
    S #= S0 - 1,
    val_resp(Response, S).

