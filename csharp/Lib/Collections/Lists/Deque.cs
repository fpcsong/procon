namespace Procon
{
    using System;
    using System.Collections;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.Linq;
    using System.Text;
    using System.Threading.Tasks;

    public sealed class Deque<T>
        : IReadOnlyList<T>
    {
        private T[] _buffer = new T[0];
        private int _offset = 0;
        private int _count = 0;

        public int Count
        {
            get
            {
                return _count;
            }
        }

        public int Capacity
        {
            get
            {
                return _buffer.Length;
            }
        }

        /// <summary>
        /// Converts an index to the deque into one to the underlying buffer.
        /// </summary>
        private int Shift(int index)
        {
            Debug.Assert(Capacity > 0 && 0 <= index && index <= _count);
            return (_offset + index) % Capacity;
        }

        public T this[int index]
        {
            get
            {
                if (!(0 <= index && index < _count))
                    throw new ArgumentOutOfRangeException("index");
                return _buffer[Shift(index)];
            }
            set
            {
                if (!(0 <= index && index < _count))
                    throw new ArgumentOutOfRangeException("index");
                _buffer[Shift(index)] = value;
            }
        }

        private void Rebuild(int newCapacity)
        {
            Debug.Assert(newCapacity > _count);

            var oldBuffer = _buffer;
            var newBuffer = new T[newCapacity];

            if (_offset + _count <= oldBuffer.Length)
            {
                Array.Copy(oldBuffer, _offset, newBuffer, 0, _count);
            }
            else
            {
                var firstCount = oldBuffer.Length - _offset;
                Array.Copy(oldBuffer, _offset, newBuffer, 0, firstCount);
                Array.Copy(oldBuffer, 0, newBuffer, firstCount, _count - firstCount);
            }

            _buffer = newBuffer;
            _offset = 0;
        }

        public void EnsureCapacity(int capacity)
        {
            if (capacity <= Capacity) return;

            var grown = Capacity + Capacity / 2;
            Rebuild(Math.Max(16, Math.Max(grown, capacity)));
        }

        public void PushFront(T value)
        {
            EnsureCapacity(_count + 1);

            _offset = (_offset - 1 + Capacity) % Capacity;

            _buffer[_offset] = value;
            _count++;
        }

        public void PushBack(T value)
        {
            EnsureCapacity(_count + 1);

            _buffer[Shift(_count)] = value;
            _count++;
        }

        public T PopFront()
        {
            if (_count == 0)
                throw new InvalidOperationException("Deque is empty.");

            var i = _offset;
            _count--;
            _offset = (_offset + 1) % Capacity;

            return _buffer[i];
        }

        public T PopBack()
        {
            if (_count == 0)
                throw new InvalidOperationException("Deque is empty.");

            _count--;
            var i = Shift(_count);

            return _buffer[i];
        }

        public void Clear()
        {
            _offset = 0;
            _count = 0;
        }

        public void Shrink()
        {
            if (Capacity == _count) return;
            Rebuild(_count);
        }

        #region IEnumerable
        public IEnumerator<T> GetEnumerator()
        {
            for (var i = 0; i < Count; i++)
            {
                yield return this[i];
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
        #endregion
    }

    public static class Deque
    {
        public static Deque<X> Create<X>()
        {
            return new Deque<X>();
        }

        public static Deque<X> FromEnumerable<X>(IEnumerable<X> xs)
        {
            var deque = new Deque<X>();
            foreach (var x in xs)
            {
                deque.PushBack(x);
            }
            return deque;
        }
    }
}
