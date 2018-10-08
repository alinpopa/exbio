defmodule ExBio.Alignment.Pairwise.ScoringTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.MatchFunc
  alias ExBio.Alignment.Pairwise.Scoring

  describe "new" do
    test "validates arguments" do
      eq = MatchFunc.eq(1, 0)
      assert Scoring.new(1, 2, eq) == {:error, :invalid_args}
      assert Scoring.new(0, 2, eq) == {:error, :invalid_args}
      assert Scoring.new(1, -2, eq) == {:error, :invalid_args}
    end

    test "creates scoring" do
      eq = MatchFunc.eq(1, 0)
      {:ok, scoring} = Scoring.new(-3, 0, eq)
      assert is_reference(scoring)
    end
  end

  describe "from scores" do
    test "validates arguments" do
      assert Scoring.from_scores(1, 2, 1, 0) == {:error, :invalid_args}
      assert Scoring.from_scores(0, 2, 1, 0) == {:error, :invalid_args}
      assert Scoring.from_scores(1, -2, 1, 0) == {:error, :invalid_args}
    end

    test "creates scoring" do
      {:ok, scoring} = Scoring.from_scores(0, 0, 1, 0)
      assert is_reference(scoring)
      {:ok, scoring} = Scoring.from_scores(-1, 0, 1, 0)
      assert is_reference(scoring)
      {:ok, scoring} = Scoring.from_scores(0, -2, 1, 0)
      assert is_reference(scoring)
      {:ok, scoring} = Scoring.from_scores(-1, -2, 1, 0)
      assert is_reference(scoring)
    end
  end

  describe "from scoring" do
    test "validates arguments" do
      assert_raise ArgumentError, fn ->
        Scoring.from_scoring(%Scoring{})
      end
    end

    test "creates scoring" do
      eq = MatchFunc.eq(1, 0)
      scoring = %Scoring{match_func: eq}
      {:ok, scoring} = Scoring.from_scoring(scoring)
      assert is_reference(scoring)
    end
  end

  describe "to scoring" do
    test "validates arguments" do
      assert_raise ArgumentError, fn ->
        Scoring.to_scoring(%Scoring{})
      end
    end

    test "creates scoring" do
      eq = MatchFunc.eq(1, 0)
      {:ok, scoring} = Scoring.new(-3, -1, eq)
      {:ok, scoring} = Scoring.to_scoring(scoring)
      assert scoring.gap_open == -3
      assert scoring.gap_extend == -1
      assert is_reference(scoring.match_func)
      assert scoring.xclip_prefix < 0
      assert scoring.yclip_prefix < 0
      assert scoring.xclip_suffix < 0
      assert scoring.yclip_suffix < 0
    end
  end

  describe "xclip" do
    test "updates prefix and suffix" do
      eq = MatchFunc.eq(1, 0)
      {:ok, scoring} = Scoring.new(-3, -1, eq)
      {:ok, scoring} = Scoring.xclip(scoring, -23)
      {:ok, scoring} = Scoring.to_scoring(scoring)
      assert scoring.gap_open == -3
      assert scoring.gap_extend == -1
      assert is_reference(scoring.match_func)
      assert scoring.xclip_prefix == -23
      assert scoring.yclip_prefix < 0
      assert scoring.xclip_suffix == -23
      assert scoring.yclip_suffix < 0
    end
  end

  describe "yclip" do
    test "updates prefix and suffix" do
      eq = MatchFunc.eq(1, 0)
      {:ok, scoring} = Scoring.new(-3, -1, eq)
      {:ok, scoring} = Scoring.yclip(scoring, -23)
      {:ok, scoring} = Scoring.to_scoring(scoring)
      assert scoring.gap_open == -3
      assert scoring.gap_extend == -1
      assert is_reference(scoring.match_func)
      assert scoring.xclip_prefix < 0
      assert scoring.yclip_prefix == -23
      assert scoring.xclip_suffix < 0
      assert scoring.yclip_suffix == -23
    end
  end
end
