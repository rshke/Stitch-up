export interface Permission {
    method: "GET" | "POST" | "PUT" | "DELETE";
    path: string;
    description: string;
}

export interface Component {
    code: string;
    name: string;
    description: string;
    permissions: Permission[];
}

export const PERMISSIONS = {
    GET_MEMBERS: { method: "GET", path: "/api/members", description: "Fetch member list" },
    POST_MEMBERS: { method: "POST", path: "/api/members", description: "Create new member" },
    PUT_MEMBERS: { method: "PUT", path: "/api/members", description: "Update member" },
    GET_PROJECTS: { method: "GET", path: "/api/projects", description: "Fetch projects" },
    POST_PROJECTS: { method: "POST", path: "/api/projects", description: "Create project" },
    DELETE_PROJECTS: { method: "DELETE", path: "/api/projects", description: "Delete project" },
    GET_TASKS: { method: "GET", path: "/api/tasks", description: "Fetch tasks" },
    ARCHIVE_TASK: { method: "PUT", path: "/api/tasks/archive", description: "Archive task" },
} as const;

export const COMPONENTS: Component[] = [
    {
        code: "comp_member_list",
        name: "Member List",
        description: "View team members",
        permissions: [PERMISSIONS.GET_MEMBERS]
    },
    {
        code: "comp_member_edit",
        name: "Member Editor",
        description: "Add or edit team members",
        permissions: [PERMISSIONS.POST_MEMBERS, PERMISSIONS.PUT_MEMBERS]
    },
    {
        code: "comp_board_view",
        name: "Project Board",
        description: "View project board and tasks",
        permissions: [PERMISSIONS.GET_PROJECTS, PERMISSIONS.GET_TASKS]
    },
    {
        code: "comp_task_archive",
        name: "Task Archiver",
        description: "Archive completed tasks",
        permissions: [PERMISSIONS.ARCHIVE_TASK]
    },
    {
        code: "comp_project_edit",
        name: "Project Editor",
        description: "Add or remove projects",
        permissions: [PERMISSIONS.POST_PROJECTS, PERMISSIONS.DELETE_PROJECTS]
    }
];

export const ROLES = {
    ADMIN: {
        name: "Admin",
        components: ["comp_member_list", "comp_member_edit", "comp_board_view", "comp_task_archive", "comp_project_edit"]
    },
    MANAGER: {
        name: "Project Manager",
        components: ["comp_member_list", "comp_board_view", "comp_task_archive"]
    },
    VIEWER: {
        name: "Viewer",
        components: ["comp_member_list", "comp_board_view"]
    }
};
