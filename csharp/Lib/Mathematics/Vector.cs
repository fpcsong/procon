namespace Procon
{
    using System;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.Linq;

    public sealed class Vector
    {
        private double[] _array;

        public int Dimension => _array.Length;

        public double Abs => Math.Sqrt(_array.Select(x => x * x).Sum());

        public bool IsZero => _array.All(x => Math.Abs(x) < Eps);

        public double this[int index]
        {
            get
            {
                return 0 <= index && index < _array.Length ? _array[index] : 0;
            }
            set
            {
                Extend(index + 1);
                _array[index] = value;
            }
        }

        public void Extend(int dimension)
        {
            if (dimension < Dimension) return;

            var array = new double[dimension];
            Array.Copy(_array, array, _array.Length);
            _array = array;
        }

        public Vector Clone()
        {
            var array = new double[Dimension];
            Array.Copy(_array, array, Dimension);
            return new Vector(array);
        }

        public Vector MutateAdd(Vector r)
        {
            Extend(r.Dimension);
            for (var i = 0; i < r.Dimension; i++)
            {
                _array[i] += r[i];
            }
            return this;
        }

        public Vector MutateSub(Vector r)
        {
            Extend(r.Dimension);
            for (var i = 0; i < r.Dimension; i++)
            {
                _array[i] -= r[i];
            }
            return this;
        }

        public Vector MutateUniAdd(double r)
        {
            for (var i = 0; i < Dimension; i++)
            {
                _array[i] += r;
            }
            return this;
        }

        public Vector MutateUniSub(double r)
        {
            for (var i = 0; i < Dimension; i++)
            {
                _array[i] -= r;
            }
            return this;
        }

        public Vector MutateUniMul(double r)
        {
            for (var i = 0; i < Dimension; i++)
            {
                _array[i] *= r;
            }
            return this;
        }

        public Vector MutateUniDiv(double r)
        {
            for (var i = 0; i < Dimension; i++)
            {
                _array[i] /= r;
            }
            return this;
        }

        public double Dot(Vector r)
        {
            var l = this;
            var d = Math.Max(l.Dimension, r.Dimension);
            var s = 0.0;
            for (var i = 0; i < d; i++)
            {
                s += l[i] * r[i];
            }
            return s;
        }

        public double Angle(Vector r)
        {
            return !IsZero && !r.IsZero ? Math.Acos(Dot(r) / Abs) : 0;
        }

        public static Vector operator +(Vector l, Vector r)
        {
            return l.Clone().MutateAdd(r);
        }

        public static Vector operator +(double l, Vector r)
        {
            return r.Clone().MutateUniAdd(l);
        }

        public static Vector operator +(Vector l, double r)
        {
            return l.Clone().MutateUniAdd(r);
        }

        public static Vector operator -(Vector l, Vector r)
        {
            return l.Clone().MutateAdd(r);
        }

        public static Vector operator -(Vector l, double r)
        {
            return l.Clone().MutateUniSub(r);
        }

        public static Vector operator *(Vector l, double r)
        {
            return l.Clone().MutateUniMul(r);
        }

        public static Vector operator *(double l, Vector r)
        {
            return r.Clone().MutateUniMul(l);
        }

        public static Vector operator /(Vector l, double r)
        {
            return l.Clone().MutateUniDiv(r);
        }

        private Vector(double[] array)
        {
            _array = array;
        }

        public static Vector Zero(int dimension)
        {
            return new Vector(new double[dimension]);
        }

        public static Vector FromEnumerable(IEnumerable<double> values)
        {
            return new Vector(values.ToArray());
        }

        public static Vector From(params double[] array)
        {
            return new Vector(array);
        }

        public static double Eps = 1e-10;
    }
}
