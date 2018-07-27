[<AutoOpen>]
module ImperativeBuilderExtensions
  open System

  type Imp<'T, 'X> =
    | Step
    | Break
    | Next
    | Ret of 'T
    | Ext of 'X

  and IExt<'T, 'X, 'XW> when 'XW : struct and 'XW :> IExt<'T, 'X, 'XW> =
    abstract Run: 'X -> 'T
    abstract Combine: 'X * (unit -> Imp<'T, 'X>) -> Imp<'T, 'X>
    abstract Next: 'X * (unit -> Imp<'T, 'X>) -> Imp<'T, 'X>

  type ImperativeBuilder<'T, 'X, 'XW when 'XW : struct and 'XW :> IExt<'T, 'X, 'XW>>() =
    static let xw = Unchecked.defaultof<'XW>

    member __.Delay(f: unit -> Imp<_, _>) = f

    member __.Run(f: unit -> Imp<'T, 'X>) =
      match f () with
      | Step | Break | Next -> Unchecked.defaultof<'T>
      | Ret x -> x
      | Ext x -> xw.Run(x)

    member __.Zero() = Step
    member __.Return(r) = Ret r
    member __.ReturnFrom(m: Imp<_, _>) = m

    member __.Combine(r: Imp<_, _>, f) =
      match r with
      | Step                  -> f ()
      | Break | Next | Ret _  -> r
      | Ext x                 -> xw.Combine(x, f)

    member __.Using(x, f): Imp<_, _> = using x f
    member __.TryWith(f, h) = try f () with e -> h e
    member __.TryFinally(f, g) = try f () finally g ()

    member __.While(p, f): Imp<_, 'X> =
      let rec loop () =
        if p () then
          match f () with
          | Step | Next   -> loop ()
          | Break         -> Step
          | Ret _ as r    -> r
          | Ext x         -> xw.Next(x, f)
        else Step
      loop ()

    member this.For(xs: #seq<_>, body) =
      use e = xs.GetEnumerator()
      this.While(e.MoveNext, fun () -> body e.Current)

  [<Struct>]
  type NoExt<'T> =
    interface IExt<'T, unit, NoExt<'T>> with
      override __.Run(()) = failwith "Never"
      override __.Combine((), _) = failwith "Never"
      override __.Next((), _) = failwith "Never"

  let imp<'T> = ImperativeBuilder<'T, unit, NoExt<'T>>()

  [<Struct>]
  type ResultExt<'T, 'E> =
    interface IExt<Result<'T, 'E>, 'E, ResultExt<'T, 'E>> with
      override __.Run(e) = Error e
      override __.Combine(e, f) = Ext e
      override __.Next(e, _) = Ext e

  let Throw e = Ext e

  let impE<'T, 'E> = ImperativeBuilder<Result<'T, 'E>, 'E, ResultExt<'T, 'E>>()
