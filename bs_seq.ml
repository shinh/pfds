(* 10.1 *)
type 'a seq = Nil | Cons of 'a * ('a * 'a) seq

let rec sizeL : 'a . 'a seq -> int = function
  | Nil -> 0
  | Cons (x, xs) -> 1 + sizeL xs

let rec sizeS : 'a . 'a seq -> int = function
  | Nil -> 0
  | Cons (x, xs) -> 1 + 2 * sizeS xs

let test =
  let s = Nil in
  assert (0 == sizeL s);
  assert (0 == sizeS s);

  let s = Cons (1, Nil) in
  assert (1 == sizeL s);
  assert (1 == sizeS s);

  let s = Cons (1,  Cons ((2, 3), Nil)) in
  assert (2 == sizeL s);
  assert (3 == sizeS s);

  let s = Cons (1,  Cons ((2, 3), Cons (((4, 5), (6, 7)), Nil))) in
  assert (3 == sizeL s);
  assert (7 == sizeS s);

