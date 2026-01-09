import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddEditUser } from './add-edit-user';

describe('AddEditUser', () => {
  let component: AddEditUser;
  let fixture: ComponentFixture<AddEditUser>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddEditUser]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddEditUser);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
