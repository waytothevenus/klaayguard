import { toast } from "react-toastify";

export const DECIMALS = 100000000000;

export const notify = (message: string, type: string) => {
  switch (type) {
    case "error":
      toast.error(message);
      break;
    case "info":
      toast.info(message);
      break;
    case "success":
      toast.success(message);
      break;
    default:
      toast.info(message);
  }
};
