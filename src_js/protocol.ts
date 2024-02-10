export interface Request {
  command: string;
  seq: number;
  arguments?: {};
}

export interface Message {
  seq: number;
  type: 'response' | 'event';
}

export interface Response extends Message {
  seq: number;
  type: 'response';
  command: string;
  request_seq: number;
  success: boolean;
  body?: {};
  message?: string;
}

export interface Event extends Message {
  seq: number;
  type: 'event';
  event: string;
  body: {};
}
