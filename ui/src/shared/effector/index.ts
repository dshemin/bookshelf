export const setupEffector = async(env: string): Promise<void> => {
    if (env === "production") {
        return;
    }

    const module = await import("effector-logger");

    module.attachLogger({
        name: "bookshelf",
    });
};
