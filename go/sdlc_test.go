package sdlc

import "testing"

func TestVersion(t *testing.T) {
	if Version != "0.0.1" {
		t.Errorf("expected 0.0.1, got %s", Version)
	}
}
