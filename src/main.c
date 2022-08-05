#include <stdio.h>
#include <sys/inotify.h>
#include <thread> //for multithreading
#include <future> //for return-value of started sthreads

#include "file-listener.h"



#define MAX_EVENTS 1024  /* Maximum number of events to process*/
#define LEN_NAME 16
#define EVENT_SIZE  ( sizeof (struct inotify_event) ) /*size of one event*/
#define BUF_LEN     ( MAX_EVENTS * ( EVENT_SIZE + LEN_NAME ))
int fd, wd;



void sig_handler(int sig){
       /* Step 5. Remove the watch descriptor and close the inotify instance*/
       inotify_rm_watch( fd, wd );
       close( fd );
       exit( 0 );
}



int main(int argc, char **argv){

	std::promise<int> web_server;
	auto buf = web_server.get_future();
	std::thread t(&func, std::move(web_server));
	t.join();
	string i = buf.get();

	printf("Started webserver successfully");
	start_rest(); //rust rest-api

	fd = inotify_init();
	if (fd < 0)
        	perror ("inotify_init");

	wd = inotify_add_watch(fd,
        	        "/home/fabiane/coding/c/file-listener/",
                	IN_MODIFY | IN_CLOSE_WRITE );
	if (wd < 0)
        	perror ("inotify_add_watch");

	while(1){
		int i=0,length;
              	char buffer[BUF_LEN];
 
              	/* Step 3. Read buffer*/
              	length = read(fd,buffer,BUF_LEN);
 
	        /* Step 4. Process the events which has occurred */
              	while(i<length){
 
		struct inotify_event *event = (struct inotify_event *) &buffer[i];
 
                	if(event->len){
                		if ( event->mask & IN_CREATE ) {
                   		if ( event->mask & IN_ISDIR ) {
                     			printf( "The directory %s was created.\n", event->name );
                     		} else {
                       			printf( "The file %s was created.\n", event->name );
                    		}
                    		} else if ( event->mask & IN_DELETE ) {
                    			if ( event->mask & IN_ISDIR ) {
                      				printf( "The directory %s was deleted.\n", event->name );
                    			} else {
                      				printf( "The file %s was deleted.\n", event->name );
                    			}
                    		} else if ( event->mask & IN_MODIFY ) {
                    			if ( event->mask & IN_ISDIR ) {
                      				printf( "The directory %s was modified.\n", event->name );
                    			} else {
                     				printf( "The file %s was modified.\n", event->name );
						if (event->name == "test-file") {
							//call rust function
							render_nm();
						}
                    			}
                    		}
                	}
        		i += EVENT_SIZE + event->len;
          	}
    	}
}
