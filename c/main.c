#include <nanomsg/nn.h>
#include <nanomsg/reqrep.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

#define URL "ipc:///tmp/a.ipc"

void reply(int sock, const char *msg);


int main(const int args, const char ** argv)
{
    int socket = nn_socket(AF_SP, NN_REP);
    assert(socket >= 0);

    printf("Binding to address '%s'\n", URL);
    assert(nn_bind(socket, URL) >= 0);

    while (1)
    {
        printf("Waiting for a message\n");
        char *buffer = NULL;
        int bytes = nn_recv(socket, &buffer, NN_MSG, 0);
        assert(bytes >= 0);
        printf("Received '%s'\n", buffer);

        if(!strcmp(buffer, "PING"))
        {
            reply(socket, "PONG");
        }
        else if (!strcmp(buffer, "STOP"))
        {
            reply(socket, "OK");
            break;
        }
        else
            reply(socket, "UNKNOWN REQUEST");

        nn_freemsg(buffer);
    }

    return nn_shutdown(socket, 0);
}

void reply(int sock, const char *msg)
{
    int msglen = strlen(msg) + 1;
    int bytes = nn_send(sock, msg, msglen, 0);
    assert(bytes == msglen);
    printf("Replied with '%s'\n", msg);
}
