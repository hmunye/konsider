import { useNavigate } from "@tanstack/react-router";
import { logOut } from "../../api/auth";
import Logo from "../../assets/images/logo.png";
import { Button } from "./button";

export default function Navbar() {
  const navigate = useNavigate();

  const handleLogOut = async () => {
    const response = await logOut();

    console.log(response);

    if (response.error) {
      console.log(response.error);
    } else {
      navigate({ to: "/" });
    }
  };

  return (
    <nav className="flex w-full justify-between items-center border-b border-b-foreground/10 p-2 px-10 lg:px-20">
      <div className="flex justify-evenly items-center">
        <img src={Logo} alt="Konsider Logo" width={50} />
        <span className="text-3xl font-nippo-bold">Konsider</span>
      </div>
      <div className="mt-2">
        <Button onClick={handleLogOut} className="bg-muted text-sm">
          Log Out
        </Button>
      </div>
    </nav>
  );
}
