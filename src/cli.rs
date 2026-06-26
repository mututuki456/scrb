use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum Command {
    #[bpaf(command)]
    Md {
        #[bpaf(positional("PROJECT"))]
        project: String,

        #[bpaf(positional("PAGE"))]
        page: String,
    },

    #[bpaf(command)]
    Raw {
        #[bpaf(positional("PROJECT"))]
        project: String,

        #[bpaf(positional("PAGE"))]
        page: String,
    },

    #[bpaf(command("ls"))]
    List {
        #[bpaf(positional("PROJECT"))]
        project: String,
        
        #[bpaf(positional("KEYWORD"))]
        keyword: Option<String>,
    },
}
