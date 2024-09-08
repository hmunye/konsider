import LogoImage from "../../assets/logo.png";

export function Logo() {
  return (
    <div className="flex flex-row items-center">
      <img src={LogoImage} alt="Konsider Logo" width={230} height={230} />
    </div>
  );
}
