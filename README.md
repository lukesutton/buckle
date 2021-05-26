```
######  #     #  #####  #    # #       ####### 
#     # #     # #     # #   #  #       #       
#     # #     # #       #  #   #       #       
######  #     # #       ###    #       #####   
#     # #     # #       #  #   #       #       
#     # #     # #     # #   #  #       #       
######   #####   #####  #    # ####### ####### 
```

## What Buckle

A terminal user interface library. It is intended for immediate-mode rendering, where layouts are defined using constraints rather than fixed positions.

It uses a flow style of layout, but is simpler than many other options. Elements in a layout may have their sizing — in either axis — set to:

- Fixed; a fixed size that will always be respected
- Hug; if the element is a container, it will derive it's size from the maximum required to accomodate its children
- Fill; takes up as much space as it can within it's parent container

There are some rules for the application of these settings:

- Fills take up the space remaining after all other elements in a container have been sized, with the remaining space shared between all fills
- If a container cannot accomodate all of it's children based on their sizing, layout proceeds as normal — left to right, top to bottom — but any children falling outside of bounds are either truncated or clipped entirely.
- If a container specifies a Hug sizing on an axis, but all it's children specify Fill on the same axis, the resulting size is always 0; this may be turned into a panic in the future

Each element may decide how it's contents are aligned for each axis:

- Start
- End
- Center

Child elements may also be given spacing:

- Packed; no spacing
- Fixed; an arbitrary amount, which can result in elements being truncated or clipped
- Even; calculated spacings where alignment is ignored, which expand or collapse depending on the size of the parent element

## Philosophy

The general approach is to keep the number of configuration options to a minimumm, along with simple rules for calculating the layout. The expectation is that more complex layouts can be achieved via the composition of layouts, rather than giving them complex options and behaviour.

## Differences

Some notable differences to other layout systems:

- Has no rules for expanding or collapsing elements based on some weighting
- Makes no attempts to accomodate all elements; if they won't fit they get truncated or clipped entirely
- Has no methods for shifting child-elements on an axis; instead container elements should be used
