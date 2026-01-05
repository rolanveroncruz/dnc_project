import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {User, UserService} from '../../../api_services/user-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {MatCard, MatCardHeader, MatCardContent,MatCardTitle, MatCardSubtitle} from '@angular/material/card';
import {AddEditRolePermissions} from '../setup-roles/add-edit-role-permissions/add-edit-role-permissions';
import {MatDialog} from '@angular/material/dialog';
import {AddEditUser} from './add-edit-user/add-edit-user';

type LoadState = 'loading' | 'loaded' | 'error';
@Component({
  selector: 'app-setup-users',
  imports: [
    MatCard,
    MatCardHeader,
    MatCardContent,
    GenericDataTableComponent,
    MatCardTitle,
    MatCardSubtitle
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

  private dialog = inject(MatDialog);

  ngOnInit(): void {
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
          console.log("In load(), failed to load users", err);
          this.errorMsg.set("In load(), Failed to load users");
          this.state.set('error');
        }
      })
  }
  openUserRowDialog(row:any){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditUser, {
      data:row,
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }
}
