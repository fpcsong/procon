using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace VainZero.Collections.BinarySearchTrees
{
    /*
    参考 https://www.slideshare.net/iwiwi/2-12188757

    この実装にある merge は列の連結であり、これを使うと平衡二分探索ツリーにならない (ただの平衡二分ツリーになる)。
    17ページにある「たいてい、列を管理するために使われる」の意味が分からるようになるまで保留。
    */

    public sealed class Treap<TKey, TValue>
    {
        public sealed class Node
        {
            public readonly bool HasValue;
            public TKey Key { get; set; }
            public TValue Value { get; set; }
            public double Priority { get; set; }
            public Node Left { get; set; }
            public Node Right { get; set; }
            public int Count { get; set; }

            public bool IsEmpty
            {
                get
                {
                    return !HasValue;
                }
            }

            public override string ToString()
            {
                if (IsEmpty) return "[Treap.Node: Empty]";
                return string.Concat("[Treap.Node: ", Key, " -> ", Value, "]");
            }

            void VerifyCore(IComparer<TKey> comparer, out int count, out Node leftmost, out Node rightmost)
            {
                if (IsEmpty)
                {
                    count = 0;
                    leftmost = null;
                    rightmost = null;
                }

                int leftCount, rightCount;
                Node leftLeftmost, leftRightmost, rightLeftmost, rightRightmost;
                VerifyCore(comparer, out leftCount, out leftLeftmost, out leftRightmost);
                VerifyCore(comparer, out rightCount, out rightLeftmost, out rightRightmost);

                count = leftCount + 1 + rightCount;
                leftmost = leftLeftmost ?? this;
                rightmost = rightRightmost ?? this;

                Debug.Assert(Count == count);
                Debug.Assert(leftRightmost == null || comparer.Compare(leftRightmost.Key, Key) <= 0);
                Debug.Assert(rightLeftmost == null || comparer.Compare(rightLeftmost.Key, Key) >= 0);
            }

            public void Verify(IComparer<TKey> comparer)
            {
                int count;
                Node leftmost, rightmost;
                VerifyCore(comparer, out count, out leftmost, out rightmost);
            }

            public Node OnUpdated()
            {
                Count = Left.Count + Right.Count + 1;
                return this;
            }

            static Node Merge(Node l, Node r)
            {
                if (l.IsEmpty) return r;
                if (r.IsEmpty) return l;

                if (l.Priority > r.Priority)
                {
                    l.Right = Merge(l.Right, r);
                    return l.OnUpdated();
                }
                else
                {
                    r.Left = Merge(l, r.Left);
                    return r.OnUpdated();
                }
            }

            public Node Merge(Node other)
            {
                return Merge(this, other);
            }

            public void Split(int index, out Node first, out Node second)
            {
                if (IsEmpty)
                {
                    first = second = this;
                    return;
                }

                var leftCount = Left.Count;
                if (index <= leftCount)
                {
                    Node leftSecond;
                    Left.Split(index, out first, out leftSecond);
                    Left = leftSecond;
                    second = OnUpdated();
                }
                else
                {
                    Node rightFirst;
                    Right.Split(index - (leftCount + 1), out rightFirst, out second);
                    Right = rightFirst;
                    first = OnUpdated();
                }
            }

            public bool TryFindNode(TKey key, IComparer<TKey> comparer, out Node node)
            {
                if (IsEmpty)
                {
                    node = null;
                    return false;
                }

                var comparison = comparer.Compare(key, Key);
                if (comparison == 0)
                {
                    node = this;
                    return true;
                }
                else if (comparison < 0)
                {
                    return Left.TryFindNode(key, comparer, out node);
                }
                else
                {
                    return Right.TryFindNode(key, comparer, out node);
                }
            }

            public int LowerBound(TKey target, IComparer<TKey> comparer)
            {
                if (IsEmpty) return 0;

                var comparison = comparer.Compare(target, Key);
                if (comparison <= 0)
                {
                    return Left.LowerBound(target, comparer);
                }
                else
                {
                    return Left.Count + 1 + Right.LowerBound(target, comparer);
                }
            }

            public int UpperBound(TKey target, IComparer<TKey> comparer)
            {
                if (IsEmpty) return 0;

                var comparison = comparer.Compare(target, Key);
                if (comparison < 0)
                {
                    return Left.UpperBound(target, comparer);
                }
                else
                {
                    return Left.Count + 1 + Right.UpperBound(target, comparer);
                }
            }

            public bool Nth(int index, out Node node)
            {
                if (IsEmpty)
                {
                    node = null;
                    return false;
                }

                if (index == 0)
                {
                    node = this;
                    return true;
                }

                var leftCount = Left.Count;
                if (leftCount <= index)
                {
                    return Left.Nth(index, out node);
                }
                else
                {
                    return Right.Nth(index - (leftCount + 1), out node);
                }
            }

            public void ToArrayCore(List<KeyValuePair<TKey, TValue>> list)
            {
                if (IsEmpty) return;
                Left.ToArrayCore(list);
                list.Add(new KeyValuePair<TKey, TValue>(Key, Value));
                Right.ToArrayCore(list);
            }

            public KeyValuePair<TKey, TValue>[] ToArray()
            {
                var list = new List<KeyValuePair<TKey, TValue>>(Count);
                ToArrayCore(list);
                return list.ToArray();
            }

            public Treap<TKey, Y>.Node Map<Y>(Func<TKey, TValue, Y> selector)
            {
                if (IsEmpty)
                {
                    return Treap<TKey, Y>.Node.Empty;
                }
                return new Treap<TKey, Y>.Node(HasValue, Key, selector(Key, Value), Priority, Left.Map(selector), Right.Map(selector), Count);
            }

            public Node(bool hasValue, TKey key, TValue value, double priority, Node left, Node right, int count)
            {
                HasValue = hasValue;
                Key = key;
                Value = value;
                Priority = priority;
                Left = left;
                Right = right;
                Count = count;
            }

            static Node CreateEmpty()
            {
                var node = new Node(false, default(TKey), default(TValue), default(double), null, null, 0);
                node.Left = node;
                node.Right = node;
                return node;
            }

            public static readonly Node Empty = CreateEmpty();
        }

        public Node Root { get; set; }
        public readonly IComparer<TKey> Comparer;
        public readonly Random Random;

        public override string ToString()
        {
            return string.Concat("[Treap: Count = ", Count, "]");
        }

        public int Count
        {
            get
            {
                return Root.Count;
            }
        }

        public KeyValuePair<TKey, TValue>[] ToArray()
        {
            return Root.ToArray();
        }

        public int LowerBound(TKey key)
        {
            return Root.LowerBound(key, Comparer);
        }

        public int UpperBound(TKey key)
        {
            return Root.UpperBound(key, Comparer);
        }

        public bool TryGetValue(TKey key, out TValue value)
        {
            Node node;
            if (Root.TryFindNode(key, Comparer, out node))
            {
                value = node.Value;
                return true;
            }

            value = default(TValue);
            return false;
        }

        Node ForceFindNode(TKey key)
        {
            Node node;
            if (!Root.TryFindNode(key, Comparer, out node))
            {
                throw new KeyNotFoundException("Key not found: " + key);
            }
            return node;
        }

        public TValue ForceGet(TKey key)
        {
            return ForceFindNode(key).Value;
        }

        public void AddOrUpdate(TKey key, TValue value)
        {
            Node node;
            if (Root.TryFindNode(key, Comparer, out node))
            {
                node.Value = value;
            }
            else
            {
                Add(key, value);
            }
        }

        public void Add(TKey key, TValue value)
        {
            Root = Root.Merge(SingletonNode(key, value));
        }

        public void RemoveAt(int index)
        {
            Node first1, second1;
            Root.Split(index, out first1, out second1);

            Node first2, second2;
            second1.Split(1, out first2, out second2);

            Root = first1.Merge(second2);
        }

        public bool Remove(TKey key)
        {
            Node node;
            if (Root.TryFindNode(key, Comparer, out node))
            {

                return true;
            }
            return false;
        }

        public void Merge(Treap<TKey, TValue> other)
        {
            Debug.Assert(Comparer == other.Comparer);
            Root = Root.Merge(other.Root);
        }

        public void Split(int index, out Treap<TKey, TValue> first, out Treap<TKey, TValue> second)
        {
            Node l, r;
            Root.Split(index, out l, out r);
            first = WithRoot(l);
            second = WithRoot(r);
        }

        public Treap<TKey, Y> Map<Y>(Func<TKey, TValue, Y> selector)
        {
            return new Treap<TKey, Y>(Root.Map(selector), Comparer, Random);
        }

        public Treap<TKey, TValue> WithRoot(Node root)
        {
            return new Treap<TKey, TValue>(root, Comparer, Random);
        }

        public Node SingletonNode(TKey key, TValue value)
        {
            return new Node(true, key, value, Random.NextDouble(), Node.Empty, Node.Empty, 1);
        }

        public Treap(Node root, IComparer<TKey> comparer, Random random)
        {
            Root = root;
            Comparer = comparer;
            Random = random;
        }
    }

    public static partial class Treap
    {
        public static Treap<K, V> Create<K, V>(IComparer<K> comparer = null)
        {
            return new Treap<K, V>(Treap<K, V>.Node.Empty, comparer ?? Comparer<K>.Default, random: new Random());
        }

        public static Treap<K, V> FromEnumerable<K, V>(IEnumerable<KeyValuePair<K, V>> kvs, IComparer<K> comparer = null)
        {
            var t = Create<K, V>(comparer);
            foreach (var kv in kvs)
            {
                t.Add(kv.Key, kv.Value);
            }
            return t;
        }
    }
}
