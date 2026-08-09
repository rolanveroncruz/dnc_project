import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CSRDentists } from './csrdentists';

describe('CSRDentists', () => {
  let component: CSRDentists;
  let fixture: ComponentFixture<CSRDentists>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CSRDentists]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CSRDentists);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
