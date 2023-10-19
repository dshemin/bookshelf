import compose from "compose-function";
import { withAuth } from "app/providers/with-auth";
import { withConfigProvider } from "./with-config-provider";
import { withRouter } from "app/providers/with-router";

export default compose(withRouter, withAuth, withConfigProvider);
