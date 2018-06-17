namespace global
  open System
  open System.Collections
  open System.Collections.Generic

  [<AutoOpen>]
  module Operators =
    let read f = Console.ReadLine().Split([|' '|]) |> Array.map f

  module Program =
    // 入力: N
    // 出力: 解「クイーンの位置のリスト」のシーケンス
    // ルール
    // ボードは (0..N)*(0..N)
    // 行は [ [0..N] * (0..N) ]
    // 列は [ (0..N) * [0..N] ]
    // 対角線は [ [(0..N, )], [] ]
    // クイーンをN個配置する
    // 各クイーンは同じ列に配置できない、つまり for all (x1, _), (x2, _) x1 <> x2
    // 各クイーンは同じ行に配置できない、つまり for all (_, y1, (_, y2) y1 <> y2
    // 各クイーンは同じ対角線上に配置できない、つまり for all (x1, y1), (x2, y2) x1-y1 <> x2-y2 & x1+y1 <>x2+y2

    type Pos = int * int
    let solveQueensPuzzle (n: int): seq<list<Pos>> =
      let onSameRow (ly, _) (ry, _) = ly = ry
      let onSameColumn (_, lx) (_, rx) = lx = rx
      let onSameCross (ly, lx) (ry, rx) = ly + lx = ry + rx || ly - lx = ry - rx
      let conflict l r =
        onSameRow l r || onSameColumn l r || onSameCross l r
      let noConflict ls r =
        seq {
          for l in ls -> conflict l r |> not
        } |> Seq.forall id
      let poss n =
        seq {
          for y in 0..(n - 1) do
            for x in 0..(n - 1) do
              yield (y, x)
        }

      let rec go (acc: list<Pos>) (i: int) =
        seq {
          if i = n then
            yield acc
          else
            for q in poss n do
              if q |> noConflict acc then
                yield! go (q :: acc) (i + 1)
        }
      go [] 0

    [<EntryPoint>]
    let main _ =
      for solution in solveQueensPuzzle 4 do
        printfn "%A\n\n" solution
      0
