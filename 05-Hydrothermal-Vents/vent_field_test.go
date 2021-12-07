package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestAddVent(t *testing.T) {
	vf := NewVentField(10, 10)

	v := Vent{1, 1, 1, 3}

	err := vf.AddVent(v)
	require.NoError(t, err)

	require.Equal(t, 1, vf[1][1])
	require.Equal(t, 1, vf[1][2])
	require.Equal(t, 1, vf[1][3])
}
