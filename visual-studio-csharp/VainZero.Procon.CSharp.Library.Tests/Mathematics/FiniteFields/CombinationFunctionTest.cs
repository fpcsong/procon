﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace VainZero.Mathematics.FiniteFields
{
    public sealed class CombinationFunctionTest
    {
        [Fact]
        public void Test()
        {
            var m = 6;
            var mod = 7;

            var expected =
                new[]
                {
                    new[] { 1, 0, 0, 0, 0, 0, 0, },
                    new[] { 1, 1, 0, 0, 0, 0, 0, },
                    new[] { 1, 2, 1, 0, 0, 0, 0, },
                    new[] { 1, 3, 3, 1, 0, 0, 0, },
                    new[] { 1, 4, 6, 4, 1, 0, 0, },
                    new[] { 1, 5, 3, 3, 5, 1, 0, },
                    new[] { 1, 6, 1, 6, 1, 6, 1, },
                };

            var combinationFunction = new CombinationFunction(m, mod);
            var actual =
                Enumerable.Range(0, m + 1)
                .Select(n =>
                    Enumerable.Range(0, m + 1)
                    .Select(k => (int)combinationFunction.Combination(n, k))
                    .ToArray()
                ).ToArray();

            Assert.Equal(expected, actual);
        }
    }
}
