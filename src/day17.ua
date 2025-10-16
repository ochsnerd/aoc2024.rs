# Basically all code thanks to Reto

# [A,B,C,PC]
729_0_0_0
$ 0,1,5,4,3,0

⋕⊜□≠@,.
↯∞_2

Combo ← ⊡⊙(⊂0_1_2_3)

👆 ← ⍜⊡(+2)3

Xdv ← (
  ⊙(
    # evaluate the combo operand (is this needed?)
    # ⊸Combo
    # put the value of the A register 2nd from the top
    ⊙⊸⊡₀
    # right-shift
    ⍜⊙⋯↘
    :
  )
  # put back into X register
  ⍜⊡◌
  👆
)

Bxl ← (
  ⊙:⊡1.:
  ⍜∩⋯⬚0≠
  ⍜⊡◌1:
  👆
)

Bst ← (
  ⊸Combo
  ⍜⋯(⬚0↙3)
  ⍜⊡◌1:
  👆
)

Jnz ← (
  ⊙:⊡0.:
  ≠0
  ⨬(⊡3.◌|-2)
  ⍜⊡◌3:
  👆
)

Bxc ← (
  ◌
  ⊸∩⌟Combo 5 6
  ⍜∩⋯⬚0≠
  ⍜⊡◌1:
  👆
)

Out ← (
  ⊸Combo
  ⍜⋯(⬚0↙3)
  &pf@,&pf
  👆
)

# takes code + state returns state
f ← (
  # get state + current instruction + keep full instructions
  ◡(⊡÷2⊡3)
  °⊟
  ⨬(Xdv 0|Bxl|Bst|Jnz|Bxc|Out|Xdv 1|Xdv 2)
)
:

⍢(f|>⊓(÷2⊡3)⧻)