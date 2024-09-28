import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

type UserRole = "Admin" | "Reviewer" | null;

type User = {
  name: string | null;
  email: string | null;
  role: UserRole;
};

type State = {
  user: User;
  loading: boolean;
};

type Actions = {
  update: (details: Partial<User>) => void;
  clear: () => void;
  setLoading: (loading: boolean) => void;
};

export const userStore = create<State & Actions>()(
  persist(
    (set) => ({
      user: {
        name: null,
        email: null,
        role: null,
      },
      loading: true,
      update: (details) =>
        set((state) => ({
          user: { ...state.user, ...details },
        })),
      clear: () => set({ user: { name: null, email: null, role: null } }),
      setLoading: (loading) => set({ loading }),
    }),
    {
      name: "currentUser",
      storage: createJSONStorage(() => sessionStorage),
      merge: (persistedState, currentState) => ({
        ...currentState,
        ...(persistedState as State),
      }),
    },
  ),
);

userStore.getState().setLoading(false);
