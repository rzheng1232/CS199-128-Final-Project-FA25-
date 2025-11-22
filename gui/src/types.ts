export type Message = {
    user: string;
    message: string;
    timestamp: string;
}

export type Chat = {
    name: string;
    messages: Message[];
}

