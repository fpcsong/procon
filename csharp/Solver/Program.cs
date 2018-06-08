using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

public static class TemplateExtension
{
    public static X[] MakeArray<X>(this int count, Func<int, X> func)
    {
        var xs = new X[count];
        for (var i = 0; i < count; i++)
        {
            xs[i] = func(i);
        }
        return xs;
    }

    public static int[] Range(this int count, int start = 0)
    {
        return count.MakeArray(i => i + start);
    }

    public static string Intercalate<X>(this IEnumerable<X> @this, string separator)
    {
        return string.Join(separator, @this);
    }

    public sealed class ValueIndexPair<T>
        : Tuple<T, int>
    {
        public T Value { get { return Item1; } }
        public int Index { get { return Item2; } }

        public ValueIndexPair(T value, int index)
            : base(value, index)
        {
        }
    }

    public static IEnumerable<ValueIndexPair<X>> Indexed<X>(this IEnumerable<X> @this)
    {
        var i = 0;
        foreach (var x in @this)
        {
            yield return new ValueIndexPair<X>(x, i);
            i++;
        }
    }
}

public sealed class Scanner
{
    private readonly TextReader _reader;
    private readonly StringBuilder _sb = new StringBuilder();

    /// <summary>
    /// Reads next word separated by spaces.
    /// </summary>
    public string Word()
    {
        _sb.Clear();

        while (true)
        {
            var r = _reader.Read();

            if (r == ' ' || r == '\r' || r == '\n')
            {
                if (r == '\r' && _reader.Peek() == '\n')
                {
                    _reader.Read();
                }

                // Ignore leading spaces.
                if (_sb.Length == 0) continue;

                break;
            }
            else if (r == -1)
            {
                break;
            }
            else
            {
                _sb.Append((char)r);
            }
        }

        return _sb.ToString();
    }

    /// <summary>
    /// Reads next word as <see cref="int"/>.
    /// </summary>
    public int N()
    {
        return int.Parse(Word().Trim());
    }

    /// <summary>
    /// Reads next word as <see cref="long"/>.
    /// </summary>
    public long L()
    {
        return long.Parse(Word());
    }

    /// <summary>
    /// Reads next word as <see cref="double"/>.
    /// </summary>
    public double F()
    {
        return double.Parse(Word());
    }

    /// <summary>
    /// Reads next line and splits it by spaces.
    /// </summary>
    public X[] Words<X>(Func<string, X> func)
    {
        return _reader.ReadLine().Split(' ').Select(func).ToArray();
    }

    public Scanner(TextReader reader)
    {
        _reader = reader;
    }
}

public partial class Program
{
    private readonly TextReader _input;
    private readonly TextWriter _output;
    private readonly Scanner _scanner;

    private void WriteLine(int value)
    {
        _output.WriteLine(value);
    }

    private void WriteLine(long value)
    {
        _output.WriteLine(value);
    }

    private void WriteLine(double value)
    {
        _output.WriteLine(value.ToString(CultureInfo.InvariantCulture));
    }

    private void WriteLine(char value)
    {
        _output.WriteLine(value);
    }

    private void WriteLine(string value)
    {
        _output.WriteLine(value);
    }

    public Program(TextReader input, TextWriter output)
    {
        _input = input;
        _output = output;
        _scanner = new Scanner(input);
    }

    public static void Main(string[] args)
    {
        new Program(Console.In, Console.Out).EntryPoint();
    }
}

public sealed class SplayTree<TKey, TValue>
{
    readonly IComparer<TKey> Comparer;

    /// <summary>
    /// Root node. Null if empty.
    /// </summary>
    Node T;

    int CountCache;

    public int Count
    {
        get { return CountCache; }
    }

    public SplayTree(IComparer<TKey> comparer = null)
    {
        Comparer = comparer ?? Comparer<TKey>.Default;
    }

    bool TryFindNode(TKey key, out Node found)
    {
        var z = T;
        while (z != null)
        {
            var c = Comparer.Compare(z.Key, key);
            if (c < 0)
            {
                z = z.R;
            }
            else if (c > 0)
            {
                z = z.L;
            }
            else
            {
                found = z;
                return true;
            }
        }

        found = default(Node);
        return false;
    }

    public void Add(TKey newKey, TValue newValue)
    {
        var p = (Node)null;
        {
            var z = T;
            while (z != null)
            {
                p = z;
                z = Comparer.Compare(z.Key, newKey) < 0 ? z.R : z.L;
            }
        }

        {
            var z = new Node() { P = p, Key = newKey, Value = newValue };
            if (p == null)
            {
                T = z;
            }
            else if (Comparer.Compare(p.Key, newKey) < 0)
            {
                p.R = z;
            }
            else
            {
                p.L = z;
            }
            Splay(z);
        }

        CountCache++;
    }

    public bool TryRemove(TKey key, out TValue value)
    {
        Node drop;
        var found = TryFindNode(key, out drop);
        if (!found)
        {
            value = default(TValue);
            return false;
        }

        Splay(drop);

        if (drop.L == null)
        {
            Replace(drop, drop.R);
        }
        else if (drop.R == null)
        {
            Replace(drop, drop.L);
        }
        else
        {
            var rightMin = drop.R.MinNode();

            if (rightMin.P != drop)
            {
                Replace(rightMin, rightMin.R);
                rightMin.R = drop.R;
                rightMin.R.P = rightMin;
            }

            Replace(drop, rightMin);
            rightMin.L = drop.L;
            rightMin.L.P = rightMin;
        }

        CountCache--;
        value = drop.Value;
        return true;
    }

    void LeftRotate(Node x)
    {
        var y = x.R;
        if (y != null)
        {
            x.R = y.L;
            if (y.L != null) y.L.P = x;
            y.P = x.P;
        }

        if (x.P == null)
        {
            T = y;
        }
        else if (x == x.P.L)
        {
            x.P.L = y;
        }
        else
        {
            x.P.R = y;
        }

        if (y != null) y.L = x;
        x.P = y;
    }

    void RightRotate(Node x)
    {
        var y = x.L;
        if (y != null)
        {
            x.L = y.R;
            if (y.R != null) y.R.P = x;
            y.P = x.P;
        }

        if (x.P == null)
        {
            T = y;
        }
        else if (x == x.P.L)
        {
            x.P.L = y;
        }
        else
        {
            x.P.R = y;
        }

        if (y != null) y.R = x;
        x.P = y;
    }

    void Splay(Node x)
    {
        while (x.P != null)
        {
            if (x.P.P == null)
            {
                if (x.P.L == x)
                {
                    RightRotate(x.P);
                }
                else
                {
                    LeftRotate(x.P);
                }
            }
            else if (x.P.L == x && x.P.P.L == x.P)
            {
                RightRotate(x.P.P);
                RightRotate(x.P);
            }
            else if (x.P.R == x && x.P.P.R == x.P)
            {
                LeftRotate(x.P.P);
                LeftRotate(x.P);
            }
            else if (x.P.L == x && x.P.P.R == x)
            {
                RightRotate(x.P);
                LeftRotate(x.P);
            }
            else
            {
                LeftRotate(x.P);
                RightRotate(x.P);
            }
        }
    }

    void Replace(Node u, Node v)
    {
        if (u.P == null)
        {
            T = v;
        }
        else if (u == u.P.L)
        {
            u.P.L = v;
        }
        else
        {
            u.P.R = v;
        }

        if (v != null)
        {
            v.P = u.P;
        }
    }

    public KeyValuePair<TKey, TValue>[] ToArray()
    {
        var list = new List<KeyValuePair<TKey, TValue>>(Count);
        if (T != null)
        {
            T.ToList(list);
        }
        return list.ToArray();
    }

    public IEnumerator<KeyValuePair<TKey, TValue>> GetEnumerator()
    {
        foreach (var entry in ToArray())
        {
            yield return entry;
        }
    }

    sealed class Node
    {
        public TKey Key;
        public TValue Value;
        public Node P;
        public Node L;
        public Node R;

        public Node MinNode()
        {
            return L != null ? L.MinNode() : this;
        }

        public Node MaxNode()
        {
            return R != null ? R.MaxNode() : this;
        }

        public void ToList(List<KeyValuePair<TKey, TValue>> builder)
        {
            if (L != null) L.ToList(builder);
            builder.Add(new KeyValuePair<TKey, TValue>(Key, Value));
            if (R != null) R.ToList(builder);
        }
    }
}

public static class SplayTree
{
    public static SplayTree<K, V> Create<K, V>()
        where K : IComparable<K>
    {
        return new SplayTree<K, V>(Comparer<K>.Default);
    }

    public static SplayTree<K, V> ToSplayTree<T, K, V>(this IEnumerable<T> source, Func<T, K> keySelector, Func<T, V> valueSelector)
    {
        var tree = new SplayTree<K, V>(Comparer<K>.Default);
        foreach (var item in source)
        {
            tree.Add(keySelector(item), valueSelector(item));
        }
        return tree;
    }
}

public sealed partial class Program
{
    private long Solve()
    {
        return 0;
    }

    public void EntryPoint()
    {
        var I = _scanner;

        var Q = I.N();
        var T = new int[Q];
        var X = new int[Q];

        for (var i = 0; i < Q; i++)
        {
            T[i] = I.N();
            X[i] = I.N();
        }

        var tree = SplayTree.Create<int, int>();
        for (var i = 0; i < Q; i++)
        {
            var x = X[i];
            if (T[i] == 1)
            {
                tree.Add(x, 1);
            }
            else
            {
                // find node with left.count == x - 1
                // print key
                // remove it
            }
        }

        WriteLine(Solve());
    }
}
