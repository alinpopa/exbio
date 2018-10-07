defmodule ExBio.Alignment.Pairwise.MatchFuncTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.MatchFunc

  test "eq" do
    eq = MatchFunc.eq(1, 0)
    assert MatchFunc.apply(eq, 4, 5) == {:ok, 0}
    assert MatchFunc.apply(eq, 4, 4) == {:ok, 1}
  end

  test "ne" do
    ne = MatchFunc.ne(1, 0)
    assert MatchFunc.apply(ne, 4, 5) == {:ok, 1}
    assert MatchFunc.apply(ne, 4, 4) == {:ok, 0}
  end

  test "lt" do
    lt = MatchFunc.lt(1, 0)
    assert MatchFunc.apply(lt, 4, 5) == {:ok, 1}
    assert MatchFunc.apply(lt, 4, 4) == {:ok, 0}
    assert MatchFunc.apply(lt, 4, 3) == {:ok, 0}
  end

  test "lte" do
    lte = MatchFunc.lte(1, 0)
    assert MatchFunc.apply(lte, 4, 5) == {:ok, 1}
    assert MatchFunc.apply(lte, 4, 4) == {:ok, 1}
    assert MatchFunc.apply(lte, 4, 3) == {:ok, 0}
  end

  test "gt" do
    gt = MatchFunc.gt(1, 0)
    assert MatchFunc.apply(gt, 4, 5) == {:ok, 0}
    assert MatchFunc.apply(gt, 4, 4) == {:ok, 0}
    assert MatchFunc.apply(gt, 4, 3) == {:ok, 1}
  end

  test "gte" do
    gte = MatchFunc.gte(1, 0)
    assert MatchFunc.apply(gte, 4, 5) == {:ok, 0}
    assert MatchFunc.apply(gte, 4, 4) == {:ok, 1}
    assert MatchFunc.apply(gte, 4, 3) == {:ok, 1}
  end
end
