Task reminder application


Application that acts as a simple task reminder. The application implement a command line interface through which users can interact with their tasks. The application offer text help including usage information. Each task is stored persistently on the filesystem (HOME_DIR/Tasks) as a text file with an unique identifier id (UUID) as name and the '.task' extension. Tasks are identified through their unique identifiers. Each task include at least a title and a description.

The application allow users to perform the following operations on existing tasks:
   - **create** - creating a new task {UUID}.task in the HOME_DIR/Tasks folder
     
       cargo run create [title][description]
   - **remove** - removing the tasks by the specified task name or remove all the tasks marked as completed
     
       cargo run remove [task_name | completed]
   - **complete** - mark the specified task as completed
     
       cargo run complete [task_name]
   - **view** - view task content/details or view all the available tasks created
     
       cargo run view [task_name | all]
