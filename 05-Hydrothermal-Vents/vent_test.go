package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestVentFromString(t *testing.T) {
	expected := Vent{0, 9, 5, 9}

	actual, err := VentFromString("0,9 -> 5,9")
	require.NoError(t, err)
	require.Equal(t, expected, *actual)
}
