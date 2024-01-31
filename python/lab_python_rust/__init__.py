from slugify import slugify


def python_func_no_args():
    print("Hello from python_func_no_args()!")
    return 666


def python_func_one_arg(name: str) -> int:
    slug = slugify(name)
    print(f"Hello from python_func_one_arg! {slug}")
    return len(slug)


def python_func_tuple(full_name: tuple[str, str, str]) -> int:
    slug = slugify(" ".join(full_name))
    print(f"Hello from python_func_tuple! {slug}")
    return len(slug)
