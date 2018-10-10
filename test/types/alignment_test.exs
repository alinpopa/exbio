defmodule ExBio.Types.AlignmentTest do
  use ExUnit.Case
  doctest ExBio

  describe "new" do
    test "creates reference" do
      alignment = %ExBio.Types.Alignment{operations: [{:xclip, 123}], mode: :local}
      {:ok, alignment_ref} = ExBio.Types.Alignment.new(alignment)
      assert is_reference(alignment_ref)
    end

    test "raises when passing invalid data" do
      assert_raise ArgumentError, fn ->
        ExBio.Types.Alignment.new(%{wrong: :struct})
      end
    end

    test "returns invalid args when passing wrong alignment data" do
      alignment = %ExBio.Types.Alignment{operations: [{:xclip, 123}], mode: nil}
      assert ExBio.Types.Alignment.new(alignment) == {:error, :invalid_args}
    end
  end

  describe "pretty" do
    test "return pretty empty string representation when for no operations" do
      alignment = %ExBio.Types.Alignment{mode: :local}
      {:ok, alignment} = ExBio.Types.Alignment.new(alignment)

      {:ok, pretty} =
        ExBio.Types.Alignment.pretty(alignment, "CCGTCCGGCAAGGG", "AAAAACCGTTGACGGCCAA")

      assert pretty == ""
    end

    test "return pretty string representation when having non empty operations" do
      operations = [
        {:match, 0},
        {:match, 0},
        {:match, 0},
        {:subst, 0},
        {:ins, 0},
        {:ins, 0},
        {:del, 0},
        {:del, 0}
      ]

      alignment = %ExBio.Types.Alignment{mode: :local, operations: operations}
      {:ok, alignment} = ExBio.Types.Alignment.new(alignment)

      {:ok, pretty} =
        ExBio.Types.Alignment.pretty(alignment, "CCGTCCGGCAAGGG", "AAAAACCGTTGACGGCCAA")

      assert pretty == "CCGTCC--\n|||\\++xx\nAAAA--AC\n\n\n"
    end
  end

  describe "cigar" do
    test "return cigar string representation" do
      alignment = %ExBio.Types.Alignment{
        score: 5,
        xstart: 3,
        ystart: 0,
        xend: 9,
        yend: 10,
        ylen: 10,
        xlen: 10,
        mode: :semiglobal,
        operations: [
          {:match, 0},
          {:match, 0},
          {:match, 0},
          {:subst, 0},
          {:ins, 0},
          {:ins, 0},
          {:del, 0},
          {:del, 0}
        ]
      }

      {:ok, alignment} = ExBio.Types.Alignment.new(alignment)
      {:ok, cigar} = ExBio.Types.Alignment.cigar(alignment, false)
      assert cigar == "3S3=1X2I2D1S"
    end
  end

  describe "path" do
    test "return a valid path" do
      alignment = %ExBio.Types.Alignment{
        score: 5,
        xstart: 3,
        ystart: 0,
        xend: 9,
        yend: 10,
        ylen: 10,
        xlen: 10,
        mode: :semiglobal,
        operations: [
          {:match, 0},
          {:match, 0},
          {:match, 0},
          {:subst, 0},
          {:ins, 0},
          {:ins, 0},
          {:del, 0},
          {:del, 0}
        ]
      }

      {:ok, alignment} = ExBio.Types.Alignment.new(alignment)
      {:ok, path} = ExBio.Types.Alignment.path(alignment)

      assert path == [
               {4, 5, {:match, 0}},
               {5, 6, {:match, 0}},
               {6, 7, {:match, 0}},
               {7, 8, {:subst, 0}},
               {8, 8, {:ins, 0}},
               {9, 8, {:ins, 0}},
               {9, 9, {:del, 0}},
               {9, 10, {:del, 0}}
             ]
    end
  end

  describe "filter_clip_operations" do
    test "filter out xclip and yclip operations from the list of operations" do
      operations = [
        {:xclip, 10},
        {:yclip, 1}
      ]

      alignment = %ExBio.Types.Alignment{mode: :local, operations: operations}
      {:ok, alignment} = ExBio.Types.Alignment.new(alignment)

      {:ok, pretty} =
        ExBio.Types.Alignment.pretty(alignment, "CCGTCCGGCAAGGG", "AAAAACCGTTGACGGCCAA")

      assert pretty == "CCGTCCGGCA \n           \n          C\n\n\n"

      :ok = ExBio.Types.Alignment.filter_clip_operations(alignment)

      {:ok, pretty} =
        ExBio.Types.Alignment.pretty(alignment, "CCGTCCGGCAAGGG", "AAAAACCGTTGACGGCCAA")

      assert pretty == ""
    end
  end
end
