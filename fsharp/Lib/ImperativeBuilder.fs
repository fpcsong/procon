namespace global
  open System

  type Imp<'T> =
    | Step
    | Break
    | Next
    | Ret of 'T

  type ImperativeBuilder private () =
    member __.Delay(f: unit -> Imp<_>) = f

    member __.Run(f: unit -> Imp<unit>) = f () |> ignore
    member __.Run(f: unit -> Imp<_>) =
      match f () with
      | Ret x -> x
      | Step | Break | Next -> failwith "Never"

    member __.Zero() = Step
    member __.Return(r) = Ret r
    member __.ReturnFrom(m: Imp<_>) = m

    member __.Bind(m: Imp<_>, f) =
      match m with
      | Step                  -> f ()
      | Break | Next | Ret _  -> m

    member __.Combine(r: Imp<_>, f) =
      match r with
      | Step                  -> f ()
      | Break | Next | Ret _  -> r

    member __.Using(x, f): Imp<_> = using x f
    member __.TryWith(f, h) = try f () with e -> h e
    member __.TryFinally(f, g) = try f () finally g ()

    member __.While(p, f): Imp<_> =
      let rec loop () =
        if p () then
          match f () with
          | Step | Next   -> loop ()
          | Break         -> Step
          | Ret _ as r    -> r
        else Step
      loop ()

    member this.For(xs: #seq<_>, body) =
      use e = xs.GetEnumerator()
      this.While(e.MoveNext, fun () -> body e.Current)

    static member val Instance = ImperativeBuilder()
