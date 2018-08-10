module Program

open BenchmarkDotNet
open BenchmarkDotNet.Attributes
open BenchmarkDotNet.Configs
open BenchmarkDotNet.Jobs

type Benchmarks() =
  [<Benchmark>]
  member this.IsPrimeBruteForceBench() =
    ()

  [<Benchmark>]
  member this.IsPrimeBench() =
    ()

[<EntryPoint>]
let main _ =
  let config =
    let rough = AccuracyMode(MaxRelativeError = 0.1)
    let quickRoughJob = Job("QuickRough", rough, RunMode.Short)

    let c = Configs.ManualConfig()
    c.Add(quickRoughJob)

    // その他の設定をデフォルトから継承する。
    ManualConfig.Union(DefaultConfig.Instance, c)

  let _summary = Running.BenchmarkRunner.Run<Benchmarks>(config)
  0
