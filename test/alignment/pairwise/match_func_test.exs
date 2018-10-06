defmodule ExBio.Alignment.Pairwise.MatchFuncTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.MatchFunc

  test "eq" do
    eq = MatchFunc.eq()
    assert MatchFunc.apply(eq, 4, 5, {1, 0}) == {:ok, 0}
    assert MatchFunc.apply(eq, 4, 4, {1, 0}) == {:ok, 1}
  end

  test "ne" do
    ne = MatchFunc.ne()
    assert MatchFunc.apply(ne, 4, 5, {1, 0}) == {:ok, 1}
    assert MatchFunc.apply(ne, 4, 4, {1, 0}) == {:ok, 0}
  end

  test "lt" do
    lt = MatchFunc.lt()
    assert MatchFunc.apply(lt, 4, 5, {1, 0}) == {:ok, 1}
    assert MatchFunc.apply(lt, 4, 4, {1, 0}) == {:ok, 0}
    assert MatchFunc.apply(lt, 4, 3, {1, 0}) == {:ok, 0}
  end

  test "lte" do
    lte = MatchFunc.lte()
    assert MatchFunc.apply(lte, 4, 5, {1, 0}) == {:ok, 1}
    assert MatchFunc.apply(lte, 4, 4, {1, 0}) == {:ok, 1}
    assert MatchFunc.apply(lte, 4, 3, {1, 0}) == {:ok, 0}
  end

  test "gt" do
    gt = MatchFunc.gt()
    assert MatchFunc.apply(gt, 4, 5, {1, 0}) == {:ok, 0}
    assert MatchFunc.apply(gt, 4, 4, {1, 0}) == {:ok, 0}
    assert MatchFunc.apply(gt, 4, 3, {1, 0}) == {:ok, 1}
  end

  test "gte" do
    gte = MatchFunc.gte()
    assert MatchFunc.apply(gte, 4, 5, {1, 0}) == {:ok, 0}
    assert MatchFunc.apply(gte, 4, 4, {1, 0}) == {:ok, 1}
    assert MatchFunc.apply(gte, 4, 3, {1, 0}) == {:ok, 1}
  end
end
