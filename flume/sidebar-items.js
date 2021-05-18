initSidebarItems({"enum":[["RecvError","An error that may be emitted when attempting to wait for a value on a receiver when all senders are dropped and there are no more messages in the channel."],["RecvTimeoutError","An error that may be emitted when attempting to wait for a value on a receiver with a timeout when the receive operation times out or all senders are dropped and there are no values left in the channel."],["SendTimeoutError","An error that may be emitted when sending a value into a channel on a sender with a timeout when the send operation times out or all receivers are dropped."],["TryRecvError","An error that may be emitted when attempting to fetch a value on a receiver when there are no messages in the channel. If there are no messages in the channel and all senders are dropped, then `TryRecvError::Disconnected` will be returned."],["TrySendError","An error that may be emitted when attempting to send a value into a channel on a sender when the channel is full or all receivers are dropped."]],"fn":[["bounded","Create a channel with a maximum capacity."],["unbounded","Create a channel with no maximum capacity."]],"mod":[["async","Futures and other types that allow asynchronous interaction with channels."]],"struct":[["Drain","An fixed-sized iterator over the msgs drained from a channel."],["IntoIter","An owned iterator over the msgs received from a channel."],["Iter","An iterator over the msgs received from a channel."],["Receiver","The receiving end of a channel."],["SendError","An error that may be emitted when attempting to send a value into a channel on a sender when all receivers are dropped."],["Sender","A transmitting end of a channel."],["TryIter","An non-blocking iterator over the msgs received from a channel."]]});