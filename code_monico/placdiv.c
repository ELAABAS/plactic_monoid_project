/* placdiv.c
   Chris Monico, 12/5/22, including some of Daniel Brown's code from the
   paper, ``Plactic key agreement''.
   This is very ugly code, quickly written to derive and test some of the ideas in
   my paper, ``Division in the plactic monoid.''
   I would strongly recommend against re-using any of the code here as-is,
   because there is no error-checking whatsoever and a complete lack of
   documentation!
   It should compile easily with gcc; there's nothing particularly fancy in the code:

   gcc -O3 -o placdiv placdiv.c -lm

*/

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <sys/time.h>
#include <math.h>

#define USAGE "[OPTIONS], where the available options are:\n"\
"-ws <int>   : word size of |a| and |b|.\n"\
"-as <int>   : alphabet size.\n"\
"-exp <int>  : number of experiments to perform.\n"\
"--challenge : attempt to solve the challenge problem, with lifting method (Algorithm 2).\n"\
"[-lift|-nolift|-erosion]  : specify the method to use.\n"       

#define TIMINGS_FILENAME "pd-timings"

#define FIRST_SYMBOL '!'

#define MIN(_a,_b) ((_a)<(_b)?(_a):(_b))
#define MAX(_a,_b) ((_a)>(_b)?(_a):(_b))

/* Max length n=|a|=|b|, with c=ab. */
#define MAX_n 512


/* Prototypes. */
void print_word(const char *w);
int  multiply(char *res, char *a, char *b);
int  divide(char *w,char *b);
int  find_perm(char *q, char *c, char *b);


/**********************  From Brown's paper ************************/
/* The following is a modified version of the source code from
   Daniel Brown's paper, ``Plactic Key Agreement''. The code contained
   therein was condensed for the purpose of minimizing space within
   the paper. I took the liberty of slightly expanding out some typedef's
   and names, to slightly increase readability. So any lack of readability
   and/or mistakes is entirely my fault (CJM, 12/4/22). I made no structural
   changes at all because it is nice and fast, as-is.
*/
#define SWAP(a,b)(a^=b,b^=a,a^=b)
#define TAB_ALLOC_SIZE (1<<20)
#define TAB_MAX_ROWS 253

char *challenge_b="gnxkOR2uN/j/sWwxNHcMKh1DaMaV4ifNUkJhjr9WWVAHVA5FNRdNYt1/bNGYuq5lZIjIiGtxjdVl"\
                  "T+shNNf5NnYWawpQPJZStxH376j3JQgqwLLy0o5dq4vLeUJrSyoNUGfFB+dQawvYYRTQH+tJZQiA"\
                  "usuD+VTNYkqBoVnl0Vt2CDKGNhNCdiYzf7O6IhgMJVmQxgmAGUPQwOinni6asO+sqodfogSB4B0D"\
                  "g3UTUl5xnD0ALslyiSm3A7vO+8kOr8976RCTf19I+ZGWfihspGQUdcCwrcCmYRow31AEWMwbKnPL"\
                  "D41maUNHsOBVtJJU58RZcjubTZqnga1f13Bsb/lLn0rXg73vDhCEpbr+yUi6ZOYc+mZW+hB2Cvih"\
                  "6vJ3km3wxaMag86a2i+k1r9d0mcKTITJwjONvr1mDlpISCsmMwTbMcE7ddvbVjGw+TpP9xqQPaOT"\
                  "BMRkW2zP8zcc+8kAr1XClSeb3LKBriZYkHY80P7zaDJh3JJNNgwY/lpf";
char *challenge_d="xvupnymwzktxyjsuxipsuxgopstekorrxzcjloqvybhkmntxaeikmqwyYbgilpvxyzVZdhkouvvxz"\
"UXbfhnqstvxzTWaeglprsstySVZbfhmoorsxxxzRRVZacjmmnruuwwPPTXZbejkmnttuvOOQVXabdjllmnquvw"\
"MNPTUXYchkkllltuvxLMOOSVXZbgijjkqqtvwxKLMNOSWYYbhiiioopuvwJKKMMRUWXabffghjltuux"\
"IJJKLPTUWZacdefiiiksvHHIJKOQRUWXXbbccghjrtvwxGGHHILLPSSUVVWZbegijosuwyFFGGHIKNOQTUUVXaaegiinqtxxx"\
"DEEFGGJMMOOSSTWYYbeggmnouuuxCDDDFFGJLLMNRRSTWWZffgiknstvvwyABBCDEFHIKLMNOQRUVYbbdhijooootuz"\
"9AABBCEFFHJKLNOQTUVZaadfiklllmpqswx89999BCDDDHHKKLMNSTTTYaacchhhjkoqtv788889BCCCFGIIJKKPRRRUUWZZbbffgjnooqqy"\
"67777888ABCFFFGHHMNNNQSVYYYabdfhikloooorx56666777999ABEFGGJJKLMPPQRUUYaddggiimnnoswwwy"\
"4455566678889BBDDDGHILLMOOQRSTUVXYdddllnnqrrss3333344455778888BBEGGGIKLNNNNNQUVVVZccfhikmmmrsww"\
"22222233344677779AABBDDGJJJJJKLMNTTUUWdffggijknpuw1111112222223445599AAAAABCCCDGHIJPQSTUUWYZbcghilmmpvvw"\
"0000000111111223333555788ABBCCDEIJLMNNQQRRabbbccdhhijkmrrsv"\
"//////////000000111333356666789ACGJJJKMMOOSTTVZZZaaaaglmpqrrvvwwxz"\
"+++++++++++++++++++++++++/11333788BBBCDHJJJJJNNNNNOOOOOPPPTTWXYYYbcefkklpwzz";

/**********************************************************/
void insert(char **tableau, int *row_lengths, char c)
/* Perform Robinson-Schensted insertion of c into the given tableau. */
{ int i=0,j;

  for ( ; i<TAB_MAX_ROWS && row_lengths[i] && tableau[i][row_lengths[i]-1]>c; i++) {
    for (j=0; j<row_lengths[i] && tableau[i][j]<=c; j++)
      ;
    SWAP(c,tableau[i][j]);
  }
  tableau[i][row_lengths[i]++]=c;
}

/**********************************************************/
int delete(char **tableau, int *row_lengths, int peak)
/* Delete the given peak, as described in Brown's paper. */
{ int  i, j, q;
  char c=0;

  for (i=0,q=-1; q<peak; i++)
    q += (row_lengths[i]>row_lengths[i+1]);
  for (i--,SWAP(c,tableau[i][row_lengths[i]-1]), row_lengths[i]--; i--; SWAP(c,tableau[i][j]))
    for (j=row_lengths[i]-1;j>=0&&tableau[i][j]>=c;j--)
      ;
  return c;
}

/******************************************************/
int erode(char **tableau, int *row_lengths, char *b, int l)
/* Attempt to erode l from the word b, as described in the paper. */
{ int c,i,p;

  if (0==l--) 
    return 1;
  if ('\n'==b[l]) 
    return erode(tableau, row_lengths, b, l);

  for (p=i=0; i<TAB_MAX_ROWS; i++)
    p += (row_lengths[i]>row_lengths[i+1]);
  for (c=0; p-- && c<=b[l]; insert(tableau,row_lengths,c))
    if (b[l]==(c=delete(tableau,row_lengths,p)) && erode(tableau,row_lengths,b,l)) 
      return 1;
  return 0;
}

/*********************************************/
int slen(char *b)
{ int l=0;

  while(*b++)
    l++;
  return l;
}

/*********************************************/
int divide(char *w, char *b)
/* Attempt to find q such that w = qb, using erosion.
   On success: set w <-- q, and return 1.
   On failure: w is undefined, and return 0. */
{ int         i, row_size[TAB_MAX_ROWS]={}, wlen=slen(w), j;
  char       *w_tab[TAB_MAX_ROWS]; 
  static char initialized=0;
  static char *tableau_space;

  if (!(initialized)) {
    tableau_space = (char *)malloc(TAB_ALLOC_SIZE*sizeof(char));
    initialized=1;
  }
  w_tab[0] = tableau_space;


  for (i=1; i<TAB_MAX_ROWS; i++) 
    w_tab[i] = w_tab[i-1]+1+wlen/i;
  for (i=0; i<wlen; i++) 
    insert(w_tab, row_size, w[i]);

  if (erode(w_tab, row_size, b, slen(b))) {
    for (i=TAB_MAX_ROWS-1; i>=0; i--) 
      for (j=0; j<row_size[i]; j++) 
        *w++ = w_tab[i][j];
    *w=0;
    return 1;
  }
  return 0;
}

/*********************************************/
int multiply_inplace(char *w, char *b)
/* Set w <-- w*b. */
{ int         i, r[TAB_MAX_ROWS]={}, m=slen(w), n=slen(b);
  char       *d[TAB_MAX_ROWS]; 
  static char initialized=0;
  static char *tableau_space;

  if (!(initialized)) {
    tableau_space = (char *)malloc(TAB_ALLOC_SIZE*sizeof(char));
    initialized=1;
  }
  d[0] = tableau_space;

  for (i=1; i<TAB_MAX_ROWS; i++) 
    d[i]=d[i-1]+1+(m+n)/i;
  for (i=0; i<m; i++) 
    insert(d,r,w[i]);
  for (i=0; i<n; i++) 
    insert(d,r,b[i]);
  for (i=TAB_MAX_ROWS-1; i>=0; i--) 
    for(m=0; m<r[i]; m++) 
      *w++ = d[i][m]; 
  *w = 0;
  return 1;
}

/**********************  End of code from Brown's paper ************************/


/* This mess is my code. Note that there is basically no error-checking whatsoever;
   the code was written with nothing but speed in mind, so proceed with caution! 
*/
/*******************************************************/
double mill_time()
/* System time, with at least millisecond resolution. */
{ struct timeval  this_tv;
  struct timezone dumbTZ;
  double t;

  gettimeofday(&this_tv, &dumbTZ);
  t = this_tv.tv_sec + 0.000001*this_tv.tv_usec;
  return t;
}

/*********************************************/
int multiply(char *res, char *a, char *b)
{ char R[4097];

  strcpy(R, a);
  multiply_inplace(R, b);
  strcpy(res, R);
}

/*******************************************************/
void canonicalize(char *a)
{ char id[1]={0};

  multiply_inplace(a, id);
}

/*******************************************************/
void rand_word(char *a, int alen, int alphabetsize)
{ int i, x;

  for (i=0; i<alen; i++) 
    a[i] = FIRST_SYMBOL + (lrand48()%alphabetsize);
  a[i]=0;
  canonicalize(a);
}

/*********************************************************/
void print_word(const char *w)
{ int i, len=strlen(w);

  for (i=0; i<len; i++) {
    printf("%c", w[i]);
    if ((i<len-1) && (w[i]>w[i+1]))
      printf("\n");
  }
  printf("\n");
}

/***************************************************************/
void first_last_symbol(char *first, char *last, char *str)
/* Find the first (min) and last (max) symbol in the given string. */
{ char *p;

  *first = 127;
  *last = 0;
  p = str;
  while (*p) {
    *first = MIN(*first, *p);
    *last = MAX(*last, *p);
    p++;
  }
}

/******************************************************/
void delete_symbol(char *str, int loc)
/* Delete the character at the given location. */
{ char *p, *q;

  p = str+loc;
  q = str+loc+1;
  while (*q) {
    *p = *q;
     p++;
     q++;
  }
  *p = 0;
}

/******************************************************/
void insert_symbol(char *str, char c, int loc)
/* Inset the given character at the given location into 'str'. */
{ int i, j, N;
  
  N = strlen(str);
  for (i=N-1; i>=loc; i--)
    str[i+1] = str[i];
  str[loc] = c;
  str[N+1]=0;
}


/******************************************************/
void sigma_perm(char *res, char *s, int src, int dest)
/* Remove the symbol from position src, and insert it at position dest.
   No sanity checks at all are performed - very unsafe function!
*/
{ int i=0;
  char c;
  
  strcpy(res, s);
  c = s[src];
  delete_symbol(res, src);
  insert_symbol(res, c, dest);
}

/******************************************************/
void apply_rand_sigma(char *res, char *s, int N, char first, char last)
/* Choose a random 'sliding' permutation sigma and set res <-- sigma(s). */
{ int src, dest;
  
  src = lrand48()%N;
  do {
    dest = lrand48()%N;
  } while (dest == src);
  /* Delete from 'src' and reinsert at 'dest'. */
  sigma_perm(res, s, src, dest);
}

/******************************************************/
double dist(char *op1, char *op2, int n)
/* Given op1, op2: words of length n in canonical form,
   compute and return d(op1, op2), the distance from op1 to op2.
*/
{ double d=0;
  int    row=0;
  char   *r1, *r2, *r1last, *r2last, *p1, *p2;

  r1last = op1 + n-1;
  r2last = op2 + n-1;
  while (r1last >= op1) {
    /* Find the beginning of this row in op1. */
    r1 = r1last;
    while ((r1>op1) && (*(r1-1)<=*r1))
      r1--;
    /* Find the beginning of this row in op2. */
    r2 = r2last;
    while ((r2>op2) && (*(r2-1)<=*r2))
      r2--;
    /* How many symbols are in this row of op1 but not r2? */
    p1=r1;
    p2=r2;
    while (p1 <= r1last) {
      while ((p2 < r2last) && (*p2 < *p1))
        p2++;
      d += (*p1 != *p2);
      p2 += (*p1==*p2);
      p1++; 
    }
    r1last = r1-1;
    r2last = r2-1;
    row++; 
    if (r2last<op2) {
      d += (int)(r1-op1);
      return d;
    }
  }
  return d;
}

/**********************************************************/
void word_mod(char *res, char *src, char s)
/* Set res <-- src (modulo symbols s, s+1,...). 
   Or, in the notation from the paper, res <-- pi_{s-1}(src). */
{ char *p=src;

  while (*p) {
    if (*p < s) {
      *res = *p;
      res++;
    }
    p++;
  }
  *res=0;
}

/**********************************************************/
int placdiv_lifting(char *q, char *c, char *b)
/* Attempt to find a word q for which c=q*b, using the 'lifting' technique,
   Algorithm 2 in the paper. */
{ int  i, count[256], n=0;
  char c_mod[2*MAX_n+1], b_mod[MAX_n+1];
  char _b[MAX_n+1], _q[MAX_n+1], _c[2*MAX_n+1], first, last, s, *p;

  first_last_symbol(&first, &last, c);
  strcpy(_b, b);
  strcpy(_c, c);
  canonicalize(_b);
  canonicalize(_c);

  /* Count how many copies of each symbol are needed. */
  for (i=first; i<=last; i++)
    count[i] = 0;

  for (p=_c; *p; p++)
    count[*p] += 1;
  for (p=_b; *p; p++)
    count[*p] -= 1;
  /* What's left is the number of each symbol needed by q. 

     Initially, set _q to be a word with the correct number
     of copies of the first symbol. */
  n=0;
  for (i=0; i<count[first]; i++)
      _q[n++] = first;
  _q[n]=0;

  for (s=first+1; s<=last; s++) {
    // Preprend the needed number of copies of symbol s.
    for (i=n-1; i>=0; i--)
      _q[i+count[s]] = _q[i];
    for (i=0; i<count[s]; i++)
      _q[i] = s;
    n += count[s];
    _q[n]=0;

    // Compute _c mod s+1,...
    word_mod(c_mod, _c, s+1);
    // Compute _b mod s+1,...
    word_mod(b_mod, _b, s+1);
    find_perm(_q, c_mod, b_mod);
  }
  printf("\nq=%s\n", _q);
  strcpy(q, _q);
  return 1;
}

      
/**********************************************************/
int shuffle(char *s)
/* Randomly shuffle the symbols in the word s. */
{ int i, n=strlen(s), a, b;
  char t;

  for (i=0; i<n; i++) {
    a = lrand48()%n;
    b = lrand48()%n;
    t = s[a];
    s[a] = s[b];
    s[b] = t;
  }
}

/**********************************************************/
int placdiv_nolift(char *q, char *c, char *b)
/* Attempt to find a word q for which c=q*b, without lifting, 
   by applying Algorithm 1 from the paper. This just sets the problem
   up; the actual implementation of Algorithm 1 is below, as find_perm().
*/
{ int  i, count[256], n=0;
  char c_mod[2*MAX_n+1], b_mod[MAX_n+1];
  char _b[MAX_n+1], _q[MAX_n+1], _c[2*MAX_n+1], first, last, s, *p;
  
  first_last_symbol(&first, &last, c);
  strcpy(_b, b);
  strcpy(_c, c);
  canonicalize(_b);
  canonicalize(_c);
  
  /* Count how many copies of each symbol are needed. */
  for (i=first; i<=last; i++)
    count[i] = 0;
    
  for (p=_c; *p; p++)
    count[*p] += 1;
  for (p=_b; *p; p++)
    count[*p] -= 1;
  /* What's left is the number of each symbol needed by q. 
     Set _q to be a word with the correct number
     of copies of each symbol. */
  n=0;
  for (s=last; s>=first; s--) {
    for (i=0; i<count[s]; i++)
      _q[n++] = s;
  }
  _q[n]=0;

  /* Shuffle the symbols in _q. */
  shuffle(_q);

  find_perm(_q, c, b);

  strcpy(q, _q);
  return 1; 
}

/******************************************************/
long get_d1_bound(char *prod, char first, char last)
/* Return an upper bound on the number of equivalence
   classes at distance 1 from 'prod'.
   first, last: the smallest and largest symbols in prod.
*/
{ int count[256], s;
  char *p;
  long bound, fs;

  for (s=first; s<=last; s++)
    count[s] = 0;
  for (p=prod; *p; p++)
    count[*p] += 1;

  bound = 0;
  for (s=first,fs=1; s<=last; s++) {
    bound += MIN(fs, count[s])*(fs-1);
    fs += (count[s]>0);
  }
  return bound;
}


long total_jumps=0;
/******************************************************/
int find_perm(char *q, char *c, char *b)
/* Attempt to find a permutation sigma for which c=sigma(q)*b, and
   set q <-- sigma(q).
   This version assumes that q starts with a word containing the correct
   number of copies of each symbol.
   This is Algorithm 1 from the paper. */
{ char   s, first_symbol, last_symbol, *p, qp[1025], prod[2049], qb[2049], global_q[1025];
  int    i, n;
  int    d, dist_k, N, global_d;
  int    replace, clen, update;
  long   m=0, _jumps=0;
  long   max_steps;
  char   str[256];

  clen = strlen(c);
  first_last_symbol(&first_symbol, &last_symbol, c);
  /* Alphabet size? */
  { char *p=c;
    int   hash[256];
    for (i=0; i<256; i++) hash[i]=0;
    while (*p) {
      hash[*p]=1;
      p++;
    }
    for (i=first_symbol, N=0; i<=last_symbol; i++)
      N += hash[i];
  }
    

  multiply(prod, q, b);
  dist_k = (int)dist(prod, c, clen);
  n = strlen(q);
  strcpy(qb, prod);
  strcpy(global_q, q);
  global_d = dist_k;

  max_steps = 2*get_d1_bound(prod, first_symbol, last_symbol);

  while (dist_k > 0) {
    apply_rand_sigma(qp, q, n, first_symbol, last_symbol);
    multiply(prod, qp, b);
    d = (int)dist(prod, c, clen);

    m += ((int)dist(prod, qb, clen)==1);
    
    if (d <= dist_k) {
      strcpy(q, qp);
      strcpy(qb, prod);
      if (d < dist_k) {
        dist_k = d;
        m=0;
        update=1;

        if (dist_k <= global_d) {
          global_d = dist_k;
          strcpy(global_q, q);
        }
      } 
    } else if (m > max_steps) {
      _jumps++;

      /* Apply several permutations from the global best and try again. */
      strcpy(qp, global_q);
      apply_rand_sigma(qp, qp, n, first_symbol, last_symbol);
      do {
        apply_rand_sigma(qp, qp, n, first_symbol, last_symbol);
      } while (drand48()<0.5);
        
      multiply(prod, qp, b);
      d = (int)dist(prod, c, clen);
      strcpy(qb, prod);
      strcpy(q, qp);
      dist_k = d;
      m=0;
      update=1;

    }
    if (update) {
      sprintf(str, "Currently at distance %d (n=%d, N=%d, jumps=%ld)        ", d, n, N, _jumps);
      printf("%s", str); fflush(stdout);
      for (p=str; *p; p++) *p = '\b';
      printf("%s", str);
      update=0;
    }
  }
  total_jumps += _jumps;
  return 0;
}

/******************************************************/
int cmp_double(const void *a, const void *b)
{ const double *A = (double *)a, *B = (double *)b;

  if (*A < *B) return -1;
  return (*A > *B);
}

typedef struct {
  double mean;
  double std;
  double median;
  double min, max;
} stat_t;

/******************************************************/
void time_stats(stat_t *S, double *T, int N)
{ double mean, median, std;
  int    i;
  double s, s2;

  qsort(T, N, sizeof(double), cmp_double);
  s = s2 = 0;
  for (i=0; i<N; i++) {
    s += T[i];
    s2 += T[i]*T[i];
  }
  mean = s/N;
  std = s2/N - mean*mean;
  std = sqrt(std);

  if (N%2)
    median = T[N/2];
  else
    median = (T[(N/2)-1] + T[(N/2)])/2;
  S->mean = mean;
  S->std = std;
  S->median = median;
  S->min = T[0];
  S->max = T[1];
  printf("  mean: %1.5lf\n", mean);
  printf("   std: %1.5lf\n", std);
  printf("median: %1.5lf\n", median);
  printf(" range: [%1.5lf, %1.5lf]\n", T[0], T[N-1]);
}


#define MAXTESTS 100000
/***********************************************************/
double test_divisions(int size, int num_tests, long seed, int alphabet_size, int method)
/* method: 0 : rand perms with lifting
           1 : rand perms without lifting
           2 : erosion.
*/
{ char   a[1025], b[1025], c[2049], q[1029], prod[2049];
  double elapsed=0, s0, s1;
  char **b_list, **c_list, *_b, *_c;
  double *T;
  unsigned long H;
  FILE   *fp;
  stat_t  S;
  int     needs_header;
  char    filename[512], *meth[16]={"lift", "nolift", "erosion"};

  num_tests = MIN(num_tests, MAXTESTS);
  b_list = (char **)malloc(num_tests*sizeof(char*));
  c_list = (char **)malloc(num_tests*sizeof(char*));

  /* First generate all of the test cases, so that we can compare
     code modifications on the same exact words. Since the code
     uses random calls all over the place, we would otherwise
     only have the first word being the same when the same random
     seed is used. */
  _b = (char *)malloc(1025*num_tests*sizeof(char));
  _c = (char *)malloc(2049*num_tests*sizeof(char));
  if (!(_b) || !(_c)) {
    fprintf(stderr, "test_randperm(): Memory allocation error!\n");
    exit(-1);
  }
  H=0; /* Quick & simple hash so we can verify the same words were used in different runs. */
  for (int i=0; i<num_tests; i++) {
    b_list[i] = _b + i*1025;
    c_list[i] = _c + i*2049;
    rand_word(a, size, alphabet_size);
    rand_word(b_list[i], size, alphabet_size);
    multiply(c_list[i], a, b_list[i]);
    for (int j=0; j<size; j++) 
      H = ((H+1+j)*(j^b_list[i][j])) % 4611686018427387409;
    for (int j=strlen(c_list[i])-1; j>=0; j--)
      H = ((H+1+j)*(j^(11*c_list[i][j]))) % 4611686018427387409;
  }
  /* Store the time of each trial so that we can compute some stats. */
  if (!(T = (double *)malloc(num_tests*sizeof(double)))) {
    fprintf(stderr, "test_randperm(): Memory allocation error! (num_tests=%d)\n", num_tests);
    return -1.0;
  }

  for (int i=0; i<num_tests; i++) {
    s0 = mill_time();
    if (method==0)
      placdiv_lifting(q, c_list[i], b_list[i]);
    else if (method==1)
      placdiv_nolift(q, c_list[i], b_list[i]);
    else {
      strcpy(q, c_list[i]);
      divide(q,b_list[i]);
    }
      
    s1 = mill_time();

    T[i] = s1-s0;
    elapsed += T[i];

    printf("Obtained q=%s\n", q);
    multiply(prod, q, b_list[i]);
    if (strcmp(prod, c_list[i])==0) {
      printf("Verified.\n");
    } else {
      printf("Failed!!!!!!!!!!!!!!!!!!!!!!!!\n");
      printf("q=%s\n", q);
      printf("b=%s\n", b_list[i]);
      printf("q*b=%s\n", prod);
      printf("c  =%s\n", c_list[i]);
      printf("dist(q*b, c)=%1.8lf\n", dist(prod, c_list[i], strlen(prod)));
      exit(0);
    }
    printf("Elapsed time: %1.3lf seconds (mean through %d experiments: %1.3lf, jumps: %lf).\n", 
           s1-s0, i+1, elapsed/(i+1), total_jumps/(double)(i+1));
  }
  sprintf(filename, "%s.%s.%d.txt", TIMINGS_FILENAME, meth[method], alphabet_size);
  if (!(fp = fopen(filename, "r"))) {
    needs_header=1;
  } else {
    needs_header=0;
    fclose(fp);
  }

  fp = fopen(filename, "a");

  printf("\nseed=%ld, size=%d, num_tests=%d, H=%lx\n", seed, size, num_tests, H);
  time_stats(&S, T, num_tests);
  if (fp) {
    if (needs_header) 
      fprintf(fp, "size\tmean\tstd\terr\tmedian\tsamples\tseed\tH\n");
    fprintf(fp, "%3d\t%1.5lf\t%1.5lf\t%1.5lf\t%1.5lf\t%d\t%ld\t%lx\n",
            size, S.mean, S.std, S.std/sqrt((double)num_tests), S.median, num_tests, seed, H);
    fclose(fp);
  }
  free(T);
  free(_b);
  free(_c);
  return elapsed;
}

/******************************************************/
int do_testsd1(int N)
{ int     n, i, j, Nindex, d;
  double  sum_d=0, prob;
  char    q[2049], b[2049], c[4193], prod[4193], qp[2049];
  long    k, NE=1000000;

  printf("N\tn\tprobd1\n");
  n = N;
  while (n <= 1024) {
    sum_d = 0;
    for (k=0; k<NE; k++) {
      rand_word(q, n, N);
      rand_word(b, n, N);
      multiply(c, q, b);
      i = lrand48()%n;
      do {
        j = lrand48()%n;
      } while (j==i);
      sigma_perm(qp, q, i, j);
      multiply(prod, qp, b);
      d = (int)dist(prod, c, 2*n);
      if (d==1) sum_d += 1;
      if (k%100==99) {
        prob = sum_d/(k+1);
        printf("%d\t%d\t%lf\t%ld", N, n, prob, k+1);
        for (int ell=0; ell<64; ell++) printf("\b");
        fflush(stdout);
      }
    }
    prob = sum_d/NE;
    printf("%d\t%d\t%lf\n", N, n, prob);
    if (n<16*N)
      n += N;
    else if (n<32*N)
      n += 2*N;
    else if (n<64*N)
      n += 4*N;
    else
      n += 8*N;
  }
}


/******************************************************/
int main(int argC, char *args[])
{ int    i;
  long   seed = time(NULL);
  int    num_exps=200, wordsize=30, alphabetsize=16, method=0;
  double total_time=0, t0, t1;

  for (i=1; i<argC; i++) {
    if (strcmp(args[i], "-ws")==0) {
      if ((++i)<argC)
        wordsize = atoi(args[i]);
    } else if (strcmp(args[i], "-as")==0) {
      if ((++i)<argC)
        alphabetsize = atoi(args[i]);
    } else if (strcmp(args[i], "-exp")==0) {
      if ((++i)<argC)
        num_exps = atoi(args[i]);
    } else if (strcmp(args[i], "-lift")==0) {
     method=0;
    } else if (strcmp(args[i], "-nolift")==0) {
     method=1;
    } else if (strcmp(args[i], "-erosion")==0) {
     method=2;
    } else if (strcmp(args[i], "--help")==0) {
      printf("%s %s\n", args[0], USAGE);
      return 0;
    } else if (strcmp(args[i], "--tests")==0) {
      do_testsd1(alphabetsize);
      return 0;
    } else if (strcmp(args[i], "--challenge")==0) {
      char   q[513], b[513], d[1025], prod[1025];
      double s0, elapsed;

      strcpy(d, challenge_d);
      strcpy(b, challenge_b);
      s0 = mill_time();
      placdiv_lifting(q, d, b);
      elapsed = mill_time() - s0;
      printf("Obtained solution:q=\n%s\n", q);
      multiply(prod, q, b);
      if (strcmp(prod, d)==0) 
        printf("Verified.\n");
      else
        printf("An error occurred - this is NOT a solution!\n");
      
      printf("Elapsed time: %lf seconds.\n", elapsed);
      return 0;
    }
  }
  if ((alphabetsize<1) || (alphabetsize > 64) || (wordsize < 1) || 
      (wordsize > 512) || (num_exps>MAXTESTS)) {
    fprintf(stderr, "This implementation only supports:\n");
    fprintf(stderr, "       alphabetsize in 1,2,...,64.\n");
    fprintf(stderr, "           wordsize in 1,2,...,512.\n");
    fprintf(stderr, "   num. experiments in 1,2,...,%ld.\n", (long)MAXTESTS);
    return 0;
  }

  switch (method) {
    case 0: printf("Using rand perms with lifting.\n"); break;
    case 1: printf("Using rand perms without lifting.\n"); break;
    case 2: printf("Using erosion.\n"); break;
  }

  seed =  1; /* Uncomment to vary the seed. I used seed=1 for reproducability of results. */
  printf("seed = %ld\n", seed);
  srand48(seed);

  test_divisions(wordsize, num_exps, seed, alphabetsize, method);
}  

