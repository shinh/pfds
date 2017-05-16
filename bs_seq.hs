import Control.Exception

-- 10.1
data Seq a = Nil | Cons (a, Seq (a, a))

sizeL :: Seq a -> Int
sizeL Nil = 0
sizeL (Cons (x, xs)) = 1 + sizeL xs

sizeS :: Seq a -> Int
sizeS Nil = 0
sizeS (Cons (x, xs)) = 1 + 2 * sizeS xs

main = do
     let s = Nil in do
          assert (0 == (sizeL s)) return ()
          assert (0 == (sizeS s)) return ()

     let s = Cons (1, Nil) in do
          assert (1 == (sizeL s)) return ()
          assert (1 == (sizeS s)) return ()

     let s = Cons (1, Cons ((2, 3), Nil)) in do
          assert (2 == (sizeL s)) return ()
          assert (3 == (sizeS s)) return ()

     let s = Cons (1,  Cons ((2, 3), Cons (((4, 5), (6, 7)), Nil))) in do
          assert (3 == (sizeL s)) return ()
          assert (7 == (sizeS s)) return ()
