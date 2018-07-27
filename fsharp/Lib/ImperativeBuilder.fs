namespace global
  open System

  [<RequireQualifiedAccess>]
  type Imp<'T> =
    | Step
    | Break
    | Next
    | Ret of 'T

  type ImperativeBuilder private () =
    member __.Delay(f: unit -> Imp<'r>): unit -> Imp<'r> =
      f

    member __.Run(f: unit -> Imp<unit>): unit =
      f () |> ignore

    member __.Run(f: unit -> Imp<'r>): 'r =
      match f () with
      | Imp.Ret x ->
        x
      | Imp.Step
      | Imp.Break
      | Imp.Next ->
        InvalidProgramException() |> raise

    member __.Zero(): Imp<'r> =
      Imp.Step

    member __.Return(r: 'r): Imp<'r> =
      Imp.Ret r

    member __.ReturnFrom(m: Imp<'r>): Imp<'r> =
      m

    member __.Bind(m: Imp<_>, f: unit -> Imp<_>): Imp<_> =
      match m with
      | Imp.Step ->
        f ()
      | Imp.Break
      | Imp.Next
      | Imp.Ret _ ->
        m

    member __.Combine(r: Imp<'r>, f: unit -> Imp<'r>): Imp<'r> =
      match r with
      | Imp.Step ->
        f ()
      | Imp.Break
      | Imp.Next
      | Imp.Ret _ ->
        r

    member __.Using(x: 'r, f: 'r -> Imp<'y>): Imp<'y> =
      use _x = x
      f x

    member __.TryWith(f: unit -> 'r, h: exn -> 'r) =
      try
        f ()
      with
      | e -> h e

    member __.TryFinally(f: unit -> 'r, g: unit -> unit) =
      try
        f ()
      finally
        g ()

    member __.While(p: unit -> bool, f: unit -> Imp<'r>): Imp<'r> =
      let rec loop () =
        if p () then
          match f () with
          | Imp.Step
          | Imp.Next ->
            loop ()
          | Imp.Break ->
            Imp.Step
          | Imp.Ret _ as r ->
            r
        else
          Imp.Step
      loop ()

    member this.For(xs: #seq<'x>, body: 'x -> Imp<'r>): Imp<'r> =
      use e = xs.GetEnumerator()
      this.While(e.MoveNext, fun () -> body e.Current)

    static member val Instance = ImperativeBuilder()
