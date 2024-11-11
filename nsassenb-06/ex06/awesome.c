#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned int t_id;

typedef struct {
    t_id id;
    char const *name;
} t_user;

typedef struct {
    t_id next_user_id;
    t_user *users;
    size_t count;
    size_t allocated;
} t_database;

typedef enum {
    ERR_SUCCESS,
    ERR_MEMORY,
    ERR_NO_MORE_IDS,
    ERR_UNKNOWN_ID,
} e_result;

e_result create_database(t_database *database)
{
	database->next_user_id = 0;
	database->users = malloc(sizeof(t_user));
	database->count = 0;
	database->allocated = 1;
	return (ERR_SUCCESS);
}

void delete_database(t_database *database) {
	database->next_user_id = 0;
	free(database->users);
	database->users = NULL;
	database->count = 0;
	database->allocated = 0;
}

e_result create_user(t_database *database, char const *name, t_id *result) {
	if (database->count < database->allocated) {
		database->users[database->count++] = (t_user){database->next_user_id, name};
		*result = database->next_user_id++;
	} else {
		t_user* new_mem = malloc(sizeof(t_user) * database->allocated * 2);
		if (new_mem == NULL) {
			return (ERR_MEMORY);
		}
		memcpy(new_mem, database->users, database->count * sizeof(t_user));
		free(database->users);
		database->users = new_mem;
		database->allocated *= 2;
		database->users[database->count++] = (t_user){database->next_user_id, name};
		*result = database->next_user_id++;

	}
	return (ERR_SUCCESS);
}

e_result delete_user(t_database *database, t_id id) {
	for (size_t i = 0; i < database->count; i++) {
		if (database->users[i].id == id) {
			memmove(&database->users[i], &database->users[i+1], (database->count - i) * sizeof(t_user));
			return (ERR_SUCCESS);
		}
	}
	return (ERR_UNKNOWN_ID);
}

e_result get_user(t_database const *database, t_id id, t_user const **result) {
	for (size_t i = 0; i < database->count; i++) {
		if (database->users[i].id == id) {
			*result = &database->users[i];
			return (ERR_SUCCESS);
		}
	}
	return (ERR_UNKNOWN_ID);
}
