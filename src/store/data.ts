export type Connection = {
  status: string;
  mode: string;
  provider: string;
  signal: number;
};

export type Session = {
  time: number;
  totalRx: number;
  totalTx: number;
  currDown: number;
  currUp: number;
}

export type Limits = {
  down: number;
  up: number;
  limit: number;
}

export type Settings = {
  deviceIp: string;
  deviceModel: string;
  password: string;
  defaults: boolean;
}

export default class {
  connected: boolean = false;
  error: string | null = null;

  connection: Connection = {
    status: "ppp_disconnected",
    mode: "GSM",
    provider: "...",
    signal: 0
  };

  session: Session = {
    time: 0,
    totalRx: 0,
    totalTx: 0,
    currDown: 0,
    currUp: 0
  };

  limits: Limits = {
    down: 0,
    up: 0,
    limit: 0
  };

  settings: Settings = {
    deviceIp: "",
    deviceModel: "",
    password: "",
    defaults: false,
  };
}
