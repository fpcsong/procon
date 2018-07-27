namespace global
  open System

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
      | Ret x ->
        x
      | Step
      | Break
      | Next ->
        InvalidProgramException() |> raise

    member __.Zero(): Imp<'r> =
      Step

    member __.Return(r: 'r): Imp<'r> =
      Ret r

    member __.ReturnFrom(m: Imp<'r>): Imp<'r> =
      m

    member __.Bind(m: Imp<_>, f: unit -> Imp<_>): Imp<_> =
      match m with
      | Step ->
        f ()
      | Break
      | Next
      | Ret _ ->
        m

    member __.Combine(r: Imp<'r>, f: unit -> Imp<'r>): Imp<'r> =
      match r with
      | Step ->
        f ()
      | Break
      | Next
      | Ret _ ->
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
          | Step
          | Next ->
            loop ()
          | Break ->
            Step
          | Ret _ as r ->
            r
        else
          Step
      loop ()

    member this.For(xs: #seq<'x>, body: 'x -> Imp<'r>): Imp<'r> =
      use e = xs.GetEnumerator()
      this.While(e.MoveNext, fun () -> body e.Current)

    static member val Instance = ImperativeBuilder()
