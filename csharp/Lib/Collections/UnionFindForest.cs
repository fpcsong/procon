namespace Procon
{
    using System;
    using System.Collections.Generic;
    using System.Linq;
    using System.Text;
    using System.Threading.Tasks;

    public sealed class UnionFindForest
    {
        private readonly int[] _parents;
        private readonly int[] _ranks;

        /// <summary>
        /// Constructs n-vertex UFF.
        /// </summary>
        public UnionFindForest(int n)
        {
            _parents = new int[n];
            _ranks = new int[n];

            for (var v = 0; v < n; v++)
            {
                _parents[v] = v;
                _ranks[v] = 1;
            }
        }

        /// <summary>
        /// Gets the representative of the specified vertex's group.
        /// </summary>
        public int Root(int v)
        {
            if (_parents[v] == v)
            {
                return v;
            }

            var r = Root(_parents[v]);
            _parents[v] = r;
            return r;
        }

        /// <summary>
        /// Gets a value indicating whether two vertices belong to the same group.
        /// </summary>
        public bool Connects(int u, int v)
        {
            return Root(u) == Root(v);
        }

        private static void Swap<X>(ref X l, ref X r)
        {
            var t = l;
            l = r;
            r = t;
        }

        /// <summary>
        /// Merges the specified vertices' group.
        /// </summary>
        public void Merge(int u, int v)
        {
            u = Root(u);
            v = Root(v);
            if (u == v) return;

            if (_ranks[u] > _ranks[v])
            {
                Swap(ref u, ref v);
            }

            _parents[u] = v;
            _ranks[v] += _ranks[u];
        }
    }
}
