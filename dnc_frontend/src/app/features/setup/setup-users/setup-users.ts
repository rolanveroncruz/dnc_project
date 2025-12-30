import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {User, UserService} from '../../../api_services/user-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatCard, MatCardHeader, MatCardContent} from '@angular/material/card';

type LoadState = 'loading' | 'loaded' | 'error';
@Component({
  selector: 'app-setup-users',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    GenericDataTableComponent
  ],
  templateUrl: './setup-users.html',
  styleUrl: './setup-users.scss',
})
export class SetupUsers implements OnInit {
  state = signal<LoadState>('loading');
  users = signal<User[] | null>(null);
  errorMsg = signal<string | null>(null);
  private destroyRef = inject(DestroyRef);

  userColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'email', label: 'Email'},
    {key: 'role', label: 'Role'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];
  constructor(private userService: UserService) {}

  ngOnInit(): void {
    console.log('Setup Users Component Initialized');
    this.load();
  }

  private load() {
    this.state.set('loading');
    this.errorMsg.set(null)
    this.userService.getUsers()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.users.set(res.items);
          this.state.set('loaded');
        },
        error: (err) => {
          console.log(err);
          this.errorMsg.set("Failed to load clinic capabilities");
          this.state.set('error');
        }
      })
  }
}
