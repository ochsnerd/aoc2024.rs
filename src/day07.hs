{-# LANGUAGE TupleSections #-}
{-# LANGUAGE ViewPatterns #-}

import Control.Monad.Zip (mzip)
import Data.List (singleton)
import Data.List.Split (splitOn)
import Data.Tree

data Equation = Equation {result :: Int, terms :: [Int]} deriving (Show)

instance Read Equation where
  readsPrec _ (splitOn ":" -> [lhs, rhs]) =
    let result = read lhs
        terms = map read $ words rhs
     in [(Equation {result = result, terms = terms}, "")]
  readsPrec _ _ = []

data Operator = Add | Multiply | Concat | Init deriving (Show)

operators :: Tree Operator
operators = unfoldTree (,[Add, Multiply, Concat]) Init

toLayers :: [a] -> Tree a
toLayers (x : xs) = unfoldTree f (x, xs)
  where
    f (current, []) = (current, [])
    f (current, next : rest) = (current, repeat (next, rest))

evalExpr :: Int -> Operator -> Int -> Int
evalExpr _ Init a = a
evalExpr a Add b = a + b
evalExpr a Multiply b = a * b
evalExpr a Concat b = a * o + b
  where
    o = head (dropWhile (b >=) [10 ^ x | x <- [0 ..]])

interimResults :: [Int] -> Tree Int
interimResults terms = scanTree (uncurry . evalExpr) 0 (mzip operators (toLayers terms))

isSatisfyable :: Equation -> Bool
isSatisfyable (Equation r ts) = elem r $ takeWhileTree (r >=) $ interimResults ts

main = do
  contents <- readFile "input7.txt"
  let equations = (map read $ lines contents) :: [Equation]
  putStrLn "Part 2:"
  print $ sum $ map result $ filter isSatisfyable equations

scanTree :: (b -> a -> b) -> b -> Tree a -> Tree b
scanTree f s t = unfoldTree unfoldF (s, t)
  where
    unfoldF (acc, Node r sf) = (res, map (res,) sf)
      where
        res = f acc r

takeWhileTree :: (a -> Bool) -> Tree a -> Tree a
takeWhileTree p = head . unfoldForest unfoldF . singleton
  where
    unfoldF (Node r sf) = (r, filter (p . rootLabel) sf)
