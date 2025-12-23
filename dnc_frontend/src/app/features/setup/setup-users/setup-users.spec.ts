import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupUsers } from './setup-users';

describe('SetupUsers', () => {
  let component: SetupUsers;
  let fixture: ComponentFixture<SetupUsers>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupUsers]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupUsers);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
