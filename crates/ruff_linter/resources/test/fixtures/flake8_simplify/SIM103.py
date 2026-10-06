def f():
    # SIM103
    if a:
        return True
    else:
        return False


def f():
    # SIM103
    if a == b:
        return True
    else:
        return False


def f():
    # SIM103
    if a:
        return 1
    elif b:
        return True
    else:
        return False


def f():
    # SIM103
    if a:
        return 1
    else:
        if b:
            return True
        else:
            return False


def f():
    # OK
    if a:
        foo()
        return True
    else:
        return False


def f():
    # OK
    if a:
        return "foo"
    else:
        return False


def f():
    # SIM103
    if a:
        return False
    else:
        return True


def f():
    # OK
    if a:
        return False
    else:
        return False


def f():
    # OK
    if a:
        return True
    else:
        return True


def f():
    # SIM103 (but not fixable)
    def bool():
        return False
    if a:
        return True
    else:
        return False


def f():
    # SIM103
    if keys is not None and notice.key not in keys:
        return False
    else:
        return True


###
# Positive cases (preview)
###


def f():
    # SIM103
    if a:
        return True
    return False


def f():
    # SIM103
    if a:
        return False
    return True


def f():
    if not 10 < a:
        return False
    return True


def f():
    if 10 < a:
        return False
    return True


def f():
    if 10 in a:
        return False
    return True


def f():
    if 10 not in a:
        return False
    return True


def f():
    if a is 10:
        return False
    return True


def f():
    if a is not 10:
        return False
    return True


def f():
    if a == 10:
        return False
    return True


def f():
    if a != 10:
        return False
    return True


# Complete boolean chains


def homogeneous(a, b, c):
    if a:
        return True
    elif b:
        return True
    elif c:
        return True
    else:
        return False


def inverted(a, b, c):
    if a:
        return False
    elif b:
        return False
    elif c:
        return False
    else:
        return True


def issue_6184(a, b):
    if not a and b:
        return True
    elif a and b:
        return True
    else:
        return False


def precedence(a, b, c, d):
    if a if b else c:
        return True
    elif c and d:
        return True
    else:
        return False


def assignment_expression(a, b):
    if (value := a):
        return True
    elif b:
        return True
    else:
        return False


def mixed(a, b):
    if a:
        return False
    elif b:
        return True
    else:
        return False


def extra_statement(a, b):
    if a:
        foo()
        return True
    elif b:
        return True
    else:
        return False


def extra_statement_in_elif(a, b):
    if a:
        return True
    elif b:
        foo()
        return True
    else:
        return False


def non_boolean(a, b):
    if a:
        return 1
    elif b:
        return True
    else:
        return False


def commented(a, b):
    if a:
        # Preserve this explanation.
        return True
    elif b:
        return True
    else:
        return False


def shadowed(a, b, bool):
    if a:
        return True
    elif b:
        return True
    else:
        return False


def inverted_shadowed(a, b, bool):
    if a:
        return False
    elif b:
        return False
    else:
        return True


def version_guard(a):
    import sys

    if sys.version_info >= (3, 12):
        return True
    elif a:
        return True
    else:
        return False


def type_checking_guard(a):
    from typing import TYPE_CHECKING

    if TYPE_CHECKING:
        return True
    elif a:
        return True
    else:
        return False


def implicit_else_chain(a, b):
    if a:
        return True
    elif b:
        return True
    return False
