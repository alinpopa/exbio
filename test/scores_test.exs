defmodule ExBio.ScoresTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Scores

  describe "blosum62" do
    test "invalid args" do
      {:ok, f} = Scores.blosum62()
      assert {:error, :invalid_args} == Scores.apply(f, 3, 1)
    end

    test "blosum62" do
      {:ok, f} = Scores.blosum62()
      assert {:ok, 4} == Scores.apply(f, ?A, ?A)
    end
  end

  describe "pam120" do
    test "invalid args" do
      {:ok, f} = Scores.pam120()
      assert {:error, :invalid_args} == Scores.apply(f, 3, 1)
    end

    test "pam120" do
      {:ok, f} = Scores.pam120()
      assert {:ok, 3} == Scores.apply(f, ?A, ?A)
    end
  end

  describe "pam200" do
    test "invalid args" do
      {:ok, f} = Scores.pam200()
      assert {:error, :invalid_args} == Scores.apply(f, 3, 1)
    end

    test "pam200" do
      {:ok, f} = Scores.pam200()
      assert {:ok, 3} == Scores.apply(f, ?A, ?A)
    end
  end

  describe "pam250" do
    test "invalid args" do
      {:ok, f} = Scores.pam250()
      assert {:error, :invalid_args} == Scores.apply(f, 3, 1)
    end

    test "pam250" do
      {:ok, f} = Scores.pam250()
      assert {:ok, 2} == Scores.apply(f, ?A, ?A)
    end
  end

  describe "pam40" do
    test "invalid args" do
      {:ok, f} = Scores.pam40()
      assert {:error, :invalid_args} == Scores.apply(f, 3, 1)
    end

    test "pam40" do
      {:ok, f} = Scores.pam40()
      assert {:ok, 6} == Scores.apply(f, ?A, ?A)
    end
  end
end
